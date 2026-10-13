# rxchef-client

Shared local and optional HTTP execution contracts for rx-chef frontends.

```rust
use rxchef_client::{EngineClient, LocalEngineClient, export_bytes};

let client = LocalEngineClient::default();
let response = client.run("To Base64", &[0, 255], vec![])?;
assert_eq!(export_bytes(&response.result)?, b"AP8=");
# Ok::<(), rxchef_client::ClientError>(())
```

Enable `remote` to build `RemoteEngineClient`. The network server is not yet
implemented; the HTTP adapter currently has local contract-fixture tests.
The default client denies host filesystem, network and process effects, and
limits input/output to 8 MiB. Existing direct Core APIs remain unchanged.

Trace payloads are opt-in, bounded and binary-safe. The synchronous interface
belongs on a UI worker thread. Engine output limits are not process memory
limits, and an HTTP request timeout does not cancel server work.

See `docs/architecture/client.md` and `docs/development/product-roadmap.md`
in the repository for contracts, limitations and remaining work.
