# Server-Konfiguration und Betriebsmodell

`rxchef-server` ist der eigenständige Headless-Server für rx-chef. Er stellt eine HTTP-basierte REST-, JSON-RPC 2.0- und WebSocket-Schnittstelle bereit.

Transformationsaufgaben werden in isolierten Worker-Kindprozessen (`--internal-worker`) ausgeführt, um Parent-Crashs, unkontrollierte Memory-Leaks oder blockierende CPU-Schleifen zu verhindern.

## 1. Start und Grundkonfiguration

### Lokaler Loopback-Betrieb (Standard)

Standardmäßig lauscht der Server ausschließlich auf Loopback (`127.0.0.1:8787`).

```bash
# Start mit automatisch generiertem Einmal-Token (auf stderr ausgegeben, nie persistiert):
rxchef-server

# Start mit festem Token:
rxchef-server --token "mein-geheimes-token-mindestens-16-zeichen"

# Start mit Token aus Datei:
rxchef-server --token-file /pfad/zu/token.txt

# Start mit Umgebungsvariable:
RXCHEF_TOKEN="mein-geheimes-token-mindestens-16-zeichen" rxchef-server
```

### Remote-Betrieb und TLS

Der Server spricht aus Sicherheitsgründen selbst kein direktes TLS, sondern verlangt zwingend einen vorgelagerten TLS-Terminator (Reverse Proxy wie Nginx, Caddy, Envoy), sobald an ein Nicht-Loopback-Interface gebunden wird.

Zur Verhinderung von DNS-Rebinding-Angriffen muss jeder erlaubte `Host`-Header explizit konfiguriert werden.

```bash
rxchef-server \
  --listen 0.0.0.0:8787 \
  --allow-remote \
  --trust-tls-terminator \
  --allowed-host chef.internal.example.org:8787 \
  --allowed-origin https://chef.internal.example.org
```

| Flag | Typ | Standard | Beschreibung |
|---|---|---|---|
| `--listen` | SocketAddr | `127.0.0.1:8787` | Bind-Adresse. Nicht-Loopback erfordert `--allow-remote`. |
| `--allow-remote` | bool | `false` | Explizite Freigabe von Nicht-Loopback-Bindings. |
| `--trust-tls-terminator` | bool | `false` | Bestätigt, dass ein TLS-Reverse-Proxy vorgeschaltet ist. |
| `--allowed-host` | String (mehrfach) | (auto für Loopback) | Erlaubte Host-Header (DNS-Rebinding-Schutz). |
| `--allowed-origin` | String (mehrfach) | (auto für Loopback) | Erlaubte CORS-/WebSocket-Origins. |

## 2. Authentifizierung und Scopes

Jede Anfrage (außer `/api/v1/health` und initialem WebSocket-Upgrade) erfordert einen Bearer-Token:

- **Full-Access-Token (`--token`, `--token-file`, `RXCHEF_TOKEN`)**: Erlaubt alle Endpunkte, Job-Submissions und Cancellation.
- **Read-Only-Token (`--read-token`, `RXCHEF_READ_TOKEN`)**: Erlaubt nur Lese-/Discovery-Endpunkte (`version`, `capabilities`, `operations`, `describe`, `search`). Ausführungen und Jobs werden mit `403 PolicyDenied` abgelehnt.

Token-Vergleiche erfolgen in Konstantzeit (`subtle::ConstantTimeEq`), um Timing-Angriffe zu verhindern. Token werden in Debug-Ausgaben und Logs immer als `<redacted>` maskiert.

## 3. Ressourcenlimits und Worker-Supervision

```text
HTTP Client / WebSocket Client
       │
       ▼ [Axum Router & Concurrency Guard]
AppState / JobManager (Queue, Semaphore, Retention)
       │
       ▼ [Length-framed IPC stdin/stdout]
Worker-Prozess (rxchef-server --internal-worker)
       │
       ▼
LocalEngineClient / Registry / Execution Engine
```

### Konfigurierbare Grenzen

| Parameter | Standard | Beschreibung |
|---|---|---|
| `--max-body-mib` | `16` | Maximale HTTP-Requestgröße in MiB. |
| `--max-input-mib` | `8` | Maximalgröße der Eingabedaten für Recipes (max. 32 MiB durch Worker IPC). |
| `--max-output-mib` | `8` | Maximalgröße der Ausgabedaten für Recipes (max. 48 MiB durch Worker IPC). |
| `--timeout-secs` | `30` | Maximalzeit pro Job; danach wird der Worker-Prozess getötet (`SIGKILL`). |
| `--memory-mib` | `1024` | Speicherbudget pro Worker: Linux `RLIMIT_AS`, macOS gemessener Footprint, Windows Job Object. |
| `--workers` | `4` | Maximale Anzahl gleichzeitig laufender Worker-Prozesse. |
| `--max-pending` | `64` | Maximale Anzahl gepufferter + laufender Jobs (Warteschlange). |
| `--retain` | `32` | Anzahl vorgehaltener fertiger Jobs zur Ergebnisabholung. |
| `--retain-mib` | `256` | Gesamtspeicherbudget für gecachte Jobergebnisse. |
| `--retain-secs` | `900` | Vorhaltezeit fertiger Jobergebnisse (15 Minuten). |
| `--max-connections` | `256` | Maximale gleichzeitige HTTP- und WebSocket-Verbindungen. |
| `--max-ws-connections` | `64` | Maximale gleichzeitige WebSocket-Verbindungen. |
| `--max-ws-inflight-per-conn` | `8` | Maximale parallele In-Flight RPC-Anfragen pro WebSocket. |
| `--max-ws-subscriptions-per-conn` | `16` | Maximale parallele Job-Subscriptions pro WebSocket. |
| `--ws-idle-timeout-secs` | `120` | WebSocket-Timeout bei Inaktivität. |

### Plattformunterschiede bei Ressourcenlimits

- **Linux**: Durchsetzung von `RLIMIT_CPU` (Timeout + 2s Kulanz) und `RLIMIT_AS` (Adressraum/Speicher) im Pre-Exec-Hook des Kindprozesses.
- **macOS**: `RLIMIT_CPU` und Überwachung des physischen Prozessspeichers über `proc_pid_rusage` alle 5 ms. Bei Überschreitung wird der Worker beendet und geerntet. Dies ist eine überwachte Speichergrenze; kurzfristige Überschreitungen zwischen Messungen sind möglich, kein harter Adressraumdeckel. Der native Test mit sehr kleiner Grenze weist die tatsächliche Beendigung ohne Output nach.
- **BSD**: `RLIMIT_CPU`, Wall-Clock-Timeout und Request-/Response-Grenzen. Kein zusätzlicher Speicherschutz nachgewiesen.
- **Windows**: Job Object mit Prozess-/Job-Speichergrenze und `KILL_ON_JOB_CLOSE`; Zuweisung vor Übergabe der Invocation. Installation der Begrenzung muss gelingen, sonst startet keine Transformation. Wall-Clock-Timeout und Cancellation bleiben aktiv. Cross-Compile PASS; native Windows-Laufzeittests stehen aus.

## 4. Endpunkte

### REST-API (`/api/v1/*`)

- `GET  /api/v1/health` (öffentlich)
- `GET  /api/v1/version`
- `GET  /api/v1/capabilities`
- `GET  /api/v1/operations`
- `GET  /api/v1/operations/{id}`
- `POST /api/v1/validate`
- `POST /api/v1/execute`
- `POST /api/v1/recipes/execute` (Alias zu `execute`)
- `POST /api/v1/magic`
- `POST /api/v1/scan`
- `POST /api/v1/jobs`
- `GET  /api/v1/jobs/{id}`
- `POST /api/v1/jobs/{id}/cancel`

### JSON-RPC 2.0 über HTTP (`POST /api/v1/rpc`)

Unterstützt alle Methoden: `version`, `capabilities`, `operations`, `describe`, `search`, `validate`, `execute`, `magic`, `scan`, `job.submit`, `job.get`, `job.cancel`.

### WebSocket (`/api/v1/ws`)

Ermöglicht bidirektionales JSON-RPC sowie Streaming von Job-Ereignissen ohne künstliche Fortschrittswerte (`job.subscribe`).
