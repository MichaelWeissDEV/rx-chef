//! HTTP surface: REST, JSON-RPC 2.0 over HTTP, WebSocket and the security gate.
use std::{future::Future, sync::Arc, time::Duration};

use axum::{
    body::Bytes,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Request, State,
    },
    http::{header, HeaderValue, Method, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use rxchef_client::{ClientError, ErrorKind};
use serde_json::{json, Value};
use tokio::{net::TcpListener, sync::mpsc, task::JoinSet};

use crate::{
    config::{Scope, ServerConfig},
    service::{status_of, AppState, METHODS},
};

type Shared = Arc<AppState>;

/// Authentication outcome attached to each request by [`guard`].
#[derive(Clone, Copy)]
struct Auth(Option<Scope>);

const WS_AUTH_TIMEOUT: Duration = Duration::from_secs(5);

pub fn error_response(error: ClientError) -> Response {
    let status =
        StatusCode::from_u16(status_of(error.kind)).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let mut response = (status, Json(&error)).into_response();
    if error.kind == ErrorKind::Unauthorized {
        response
            .headers_mut()
            .insert(header::WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"));
    }
    response
}

fn reply(result: Result<Value, ClientError>) -> Response {
    match result {
        Ok(value) => Json(value).into_response(),
        Err(error) => error_response(error),
    }
}

fn parse_body(body: &Bytes) -> Result<Value, ClientError> {
    if body.is_empty() {
        return Ok(Value::Null);
    }
    serde_json::from_slice(body)
        .map_err(|_| ClientError::new(ErrorKind::InvalidInput, "request body is not valid JSON"))
}

fn bearer(request: &Request) -> Option<&str> {
    let value = request
        .headers()
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    let (scheme, token) = value.split_once(' ')?;
    scheme
        .eq_ignore_ascii_case("bearer")
        .then_some(token.trim())
}

fn cors(response: &mut Response, origin: Option<&HeaderValue>) {
    let headers = response.headers_mut();
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    if let Some(origin) = origin {
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
        headers.insert(header::VARY, HeaderValue::from_static("Origin"));
    }
}

/// Host / Origin / authentication / concurrency / deadline gate for every request.
async fn guard(State(state): State<Shared>, mut request: Request, next: Next) -> Response {
    let Ok(_permit) = state.connections.clone().try_acquire_owned() else {
        return error_response(ClientError::new(ErrorKind::Busy, "too many connections"));
    };
    let deny = |message: &str| error_response(ClientError::new(ErrorKind::PolicyDenied, message));
    // DNS-rebinding defence: only configured Host names are served.
    let host = request
        .headers()
        .get(header::HOST)
        .and_then(|v| v.to_str().ok())
        .map(str::to_lowercase)
        .or_else(|| request.uri().authority().map(|a| a.as_str().to_lowercase()));
    if !host.is_some_and(|h| state.config.allowed_hosts.contains(&h)) {
        return deny("host not allowed");
    }
    let origin = request.headers().get(header::ORIGIN).cloned();
    if let Some(value) = &origin {
        let allowed = value
            .to_str()
            .ok()
            .is_some_and(|o| state.config.allowed_origins.contains(&o.to_lowercase()));
        if !allowed {
            return deny("origin not allowed");
        }
    }
    if request.method() == Method::OPTIONS {
        let mut response = StatusCode::NO_CONTENT.into_response();
        let headers = response.headers_mut();
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET, POST, OPTIONS"),
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("authorization, content-type"),
        );
        headers.insert(
            header::ACCESS_CONTROL_MAX_AGE,
            HeaderValue::from_static("600"),
        );
        cors(&mut response, origin.as_ref());
        return response;
    }
    let scope = bearer(&request).and_then(|token| state.config.scope_of(token));
    let path = request.uri().path().to_owned();
    // Health is public; the WebSocket may authenticate with its first
    // message because browsers cannot set headers on upgrades.
    let public_asset = cfg!(feature = "web-ui")
        && matches!(*request.method(), Method::GET | Method::HEAD)
        && (path == "/" || path == "/index.html" || path.starts_with("/assets/"));
    let open = public_asset || path == "/api/v1/health" || path == "/api/v1/ws";
    if scope.is_none() && !open {
        let mut response = error_response(ClientError::new(
            ErrorKind::Unauthorized,
            "authentication required",
        ));
        cors(&mut response, origin.as_ref());
        return response;
    }
    request.extensions_mut().insert(Auth(scope));
    let deadline = state.config.timeout + Duration::from_secs(10);
    let mut response = if path == "/api/v1/ws" {
        next.run(request).await
    } else {
        match tokio::time::timeout(deadline, next.run(request)).await {
            Ok(response) => response,
            Err(_) => error_response(ClientError::new(
                ErrorKind::Timeout,
                "request deadline exceeded",
            )),
        }
    };
    cors(&mut response, origin.as_ref());
    response
}

pub fn router(state: Shared) -> Router {
    let limit = state.config.max_body;
    let r =
        Router::new()
            .route(
                "/api/v1/health",
                get(|| async { Json(json!({"status": "ok"})) }),
            )
            .route("/api/v1/version", get(|s, a| simple(s, a, "version")))
            .route(
                "/api/v1/capabilities",
                get(|s, a| simple(s, a, "capabilities")),
            )
            .route("/api/v1/operations", get(|s, a| simple(s, a, "operations")))
            .route(
                "/api/v1/operations/{id}",
                get(
                    |State(s): State<Shared>,
                     Extension(a): Extension<Auth>,
                     Path(id): Path<String>| async move {
                        reply(s.call(a.0, "describe", json!({"operation": id})).await)
                    },
                ),
            )
            .route(
                "/api/v1/execute",
                post(|s, a, b| body_call(s, a, "execute", b)),
            )
            .route(
                "/api/v1/recipes/execute",
                post(|s, a, b| body_call(s, a, "execute", b)),
            )
            .route(
                "/api/v1/validate",
                post(|s, a, b| body_call(s, a, "validate", b)),
            )
            .route(
                "/api/v1/graphs/validate",
                post(|s, a, b| body_call(s, a, "graph.validate", b)),
            )
            .route(
                "/api/v1/graphs/execute",
                post(|s, a, b| body_call(s, a, "graph.execute", b)),
            )
            .route("/api/v1/magic", post(|s, a, b| body_call(s, a, "magic", b)))
            .route("/api/v1/scan", post(|s, a, b| body_call(s, a, "scan", b)))
            .route(
                "/api/v1/jobs",
                post(|s, a, b| body_call(s, a, "job.submit", b)),
            )
            .route(
                "/api/v1/jobs/{id}",
                get(
                    |State(s): State<Shared>,
                     Extension(a): Extension<Auth>,
                     Path(id): Path<String>| async move {
                        reply(s.call(a.0, "job.get", json!({"id": id})).await)
                    },
                ),
            )
            .route(
                "/api/v1/jobs/{id}/cancel",
                post(
                    |State(s): State<Shared>,
                     Extension(a): Extension<Auth>,
                     Path(id): Path<String>| async move {
                        reply(s.call(a.0, "job.cancel", json!({"id": id})).await)
                    },
                ),
            )
            .route("/api/v1/rpc", post(rpc_http))
            .route("/api/v1/ws", get(websocket));

    #[cfg(feature = "web-ui")]
    let r = r.fallback(static_asset);

    #[cfg(not(feature = "web-ui"))]
    let r = r.fallback(|| async {
        error_response(ClientError::new(ErrorKind::NotFound, "no such endpoint"))
    });

    r.layer(axum::extract::DefaultBodyLimit::max(limit))
        .layer(middleware::from_fn_with_state(state.clone(), guard))
        .with_state(state)
}

#[cfg(feature = "web-ui")]
mod embedded {
    include!(concat!(env!("OUT_DIR"), "/web_assets.rs"));
}

#[cfg(feature = "web-ui")]
async fn static_asset(request: Request) -> Response {
    let path = request.uri().path();
    if !matches!(*request.method(), Method::GET | Method::HEAD)
        || path == "/api"
        || path.starts_with("/api/")
    {
        return error_response(ClientError::new(ErrorKind::NotFound, "no such endpoint"));
    }
    let path = if path == "/" { "/index.html" } else { path };
    match embedded::asset(path) {
        Some((mime, bytes)) => ([(header::CONTENT_TYPE, mime)], bytes).into_response(),
        None => error_response(ClientError::new(ErrorKind::NotFound, "no such asset")),
    }
}

async fn simple(
    State(s): State<Shared>,
    Extension(a): Extension<Auth>,
    method: &'static str,
) -> Response {
    reply(s.call(a.0, method, Value::Null).await)
}

async fn body_call(
    State(s): State<Shared>,
    Extension(a): Extension<Auth>,
    method: &'static str,
    body: Bytes,
) -> Response {
    match parse_body(&body) {
        Ok(value) => reply(s.call(a.0, method, value).await),
        Err(error) => error_response(error),
    }
}

// ---- JSON-RPC 2.0 -------------------------------------------------------

fn rpc_error(id: Value, code: i64, message: &str, data: Option<&ClientError>) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message, "data": data}})
}

fn valid_rpc(request: &Value) -> bool {
    request.get("jsonrpc") == Some(&json!("2.0"))
        && request.get("method").is_some_and(Value::is_string)
        && request
            .get("id")
            .is_none_or(|id| id.is_string() || id.is_number() || id.is_null())
        && request
            .get("params")
            .is_none_or(|params| params.is_object() || params.is_array() || params.is_null())
}

async fn send_frame(socket: &mut WebSocket, text: String) -> bool {
    matches!(
        tokio::time::timeout(
            Duration::from_secs(5),
            socket.send(Message::Text(text.into()))
        )
        .await,
        Ok(Ok(()))
    )
}

/// Handle one JSON-RPC request object; `None` for notifications.
pub async fn rpc_dispatch(state: &AppState, scope: Option<Scope>, request: Value) -> Option<Value> {
    let id = request.get("id").cloned();
    let respond_id = id.clone().unwrap_or(Value::Null);
    if !valid_rpc(&request) {
        return Some(rpc_error(Value::Null, -32600, "invalid request", None));
    }
    let method = request["method"].as_str().unwrap_or_default();
    let reply = if !METHODS.contains(&method) {
        rpc_error(respond_id, -32601, "method not found", None)
    } else {
        let params = request.get("params").cloned().unwrap_or(Value::Null);
        match state.call(scope, method, params).await {
            Ok(result) => json!({"jsonrpc": "2.0", "id": respond_id, "result": result}),
            Err(error) => {
                let code = if error.kind == ErrorKind::InvalidInput {
                    -32602
                } else {
                    -32000
                };
                rpc_error(respond_id, code, &error.message, Some(&error))
            }
        }
    };
    id.map(|_| reply)
}

async fn rpc_http(State(s): State<Shared>, Extension(a): Extension<Auth>, body: Bytes) -> Response {
    let request = match serde_json::from_slice::<Value>(&body) {
        Ok(value) if value.is_object() => value,
        Ok(_) => {
            return Json(rpc_error(
                Value::Null,
                -32600,
                "batch requests are not supported",
                None,
            ))
            .into_response()
        }
        Err(_) => return Json(rpc_error(Value::Null, -32700, "parse error", None)).into_response(),
    };
    match rpc_dispatch(&s, a.0, request).await {
        Some(reply) => Json(reply).into_response(),
        None => StatusCode::NO_CONTENT.into_response(),
    }
}

// ---- WebSocket ----------------------------------------------------------

async fn websocket(
    State(s): State<Shared>,
    Extension(a): Extension<Auth>,
    upgrade: WebSocketUpgrade,
) -> Response {
    let limit = s.config.max_body;
    let Ok(ws_permit) = s.ws_connections.clone().try_acquire_owned() else {
        return error_response(ClientError::new(
            ErrorKind::Busy,
            "too many WebSocket connections",
        ));
    };
    upgrade
        .max_message_size(limit)
        .max_frame_size(limit)
        .on_upgrade(move |socket| socket_loop(s, a.0, socket, ws_permit))
}

/// Authenticate (header already checked, else first message), then serve
/// JSON-RPC requests. `job.subscribe` streams real status transitions; no
/// progress is invented. Holds `_ws_permit` for the entire connection duration.
async fn socket_loop(
    state: Shared,
    mut scope: Option<Scope>,
    mut socket: WebSocket,
    _ws_permit: tokio::sync::OwnedSemaphorePermit,
) {
    if scope.is_none() {
        let first = tokio::time::timeout(WS_AUTH_TIMEOUT, socket.recv()).await;
        let presented = match first {
            Ok(Some(Ok(Message::Text(text)))) => serde_json::from_str::<Value>(&text)
                .ok()
                .filter(|v| v["type"] == "auth")
                .and_then(|v| v["token"].as_str().map(str::to_owned)),
            _ => None,
        };
        scope = presented.and_then(|token| state.config.scope_of(&token));
        if scope.is_none() {
            let _ = send_frame(&mut socket, json!({"type": "error", "error": ClientError::new(ErrorKind::Unauthorized, "authentication required")}).to_string()).await;
            return;
        }
    }
    if !send_frame(
        &mut socket,
        json!({"type": "ready", "protocol_version": rxchef_client::PROTOCOL_VERSION}).to_string(),
    )
    .await
    {
        return;
    }

    let (out, mut inbox) = mpsc::channel::<String>(64);
    // Dropping the set aborts in-flight requests and subscription tasks, which cancels their workers/streams.
    let mut tasks = JoinSet::new();
    let inflight_sem = Arc::new(tokio::sync::Semaphore::new(
        state.config.max_ws_inflight_per_conn,
    ));
    let active_subs = Arc::new(tokio::sync::Semaphore::new(
        state.config.max_ws_subscriptions_per_conn,
    ));
    let mut stopping = state.shutdown.subscribe();
    let mut idle_deadline = tokio::time::Instant::now() + state.config.ws_idle_timeout;

    loop {
        if *stopping.borrow() {
            break;
        }
        tokio::select! {
            _ = stopping.changed() => break,
            _ = tokio::time::sleep_until(idle_deadline) => break,
            Some(frame) = inbox.recv() => {
                if !send_frame(&mut socket, frame).await { break; }
            }
            incoming = socket.recv() => {
                idle_deadline = tokio::time::Instant::now() + state.config.ws_idle_timeout;
                let msg = match incoming {
                    Some(Ok(Message::Ping(_) | Message::Pong(_))) => continue,
                    Some(Ok(msg)) => msg,
                    // Socket closed, error, or idle timeout reached
                    _ => break,
                };
                let text = match msg {
                    Message::Text(text) => text,
                    Message::Close(_) => break,
                    _ => continue,
                };

                let request = match serde_json::from_str::<Value>(&text) {
                    Ok(request) => request,
                    Err(_) => {
                        if out.try_send(rpc_error(Value::Null, -32700, "parse error", None).to_string()).is_err() { break; }
                        continue;
                    }
                };
                if !valid_rpc(&request) {
                    if out.try_send(rpc_error(Value::Null, -32600, "invalid request", None).to_string()).is_err() { break; }
                    continue;
                }
                let subscriptions = request["method"] == "job.subscribe";
                let capacity = if subscriptions { &active_subs } else { &inflight_sem };
                let permit = match capacity.clone().try_acquire_owned() {
                    Ok(permit) => permit,
                    Err(_) => {
                        if let Some(id) = request.get("id") {
                            let error = ClientError::new(if subscriptions { ErrorKind::LimitExceeded } else { ErrorKind::Busy }, "connection capacity exhausted");
                            if out.try_send(rpc_error(id.clone(), -32000, &error.message, Some(&error)).to_string()).is_err() { break; }
                        }
                        continue;
                    }
                };
                let state = state.clone();
                let out = out.clone();
                tasks.spawn(async move { handle_frame(state, scope, request, out, permit).await; });
            }
        }
        while tasks.try_join_next().is_some() {}
    }
    // Cleanup: abort all tasks on disconnect
    tasks.abort_all();
    while tasks.join_next().await.is_some() {}
}

async fn handle_frame(
    state: Shared,
    scope: Option<Scope>,
    request: Value,
    out: mpsc::Sender<String>,
    _permit: tokio::sync::OwnedSemaphorePermit,
) {
    if request["method"] == "job.subscribe" {
        let id = request["id"].clone();
        let outcome = match request["params"]["id"].as_str() {
            Some(job) if scope == Some(Scope::Full) => {
                state.jobs.subscribe(job).map(|rx| (job.to_owned(), rx))
            }
            Some(_) => Err(ClientError::new(
                ErrorKind::PolicyDenied,
                "token scope does not permit this method",
            )),
            None => Err(ClientError::new(ErrorKind::InvalidInput, "missing job id")),
        };
        match outcome {
            Err(error) => {
                let _ = out
                    .send(rpc_error(id, -32000, &error.message, Some(&error)).to_string())
                    .await;
            }
            Ok((job, mut rx)) => {
                let _ = out
                    .send(
                        json!({"jsonrpc": "2.0", "id": id, "result": {"subscribed": job}})
                            .to_string(),
                    )
                    .await;
                loop {
                    let event = {
                        let snapshot = rx.borrow_and_update();
                        // Result payloads are fetched with job.get, not pushed.
                        json!({"type": "job", "id": snapshot.id, "status": snapshot.status,
                               "progress": snapshot.progress, "error": snapshot.error})
                    };
                    if out.send(event.to_string()).await.is_err()
                        || rx.borrow().status.is_terminal()
                    {
                        break;
                    }
                    if rx.changed().await.is_err() {
                        break;
                    }
                }
            }
        }
        return;
    }

    if let Some(reply) = rpc_dispatch(&state, scope, request).await {
        let _ = out.send(reply.to_string()).await;
    }
}

// ---- serving ------------------------------------------------------------

/// Serve until `shutdown` resolves, then cancel running jobs and drain.
pub async fn serve(
    listener: TcpListener,
    config: ServerConfig,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> std::io::Result<()> {
    let state = AppState::new(config);
    let jobs = state.jobs.clone();
    let shutdown_signal = state.shutdown.clone();
    axum::serve(listener, router(state))
        .with_graceful_shutdown(async move {
            shutdown.await;
            shutdown_signal.send_replace(true);
            jobs.shutdown();
        })
        .await
}
