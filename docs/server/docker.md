# Run the server in Docker

`docker/Dockerfile.server` builds a runtime image with the same Rust server and
worker supervisor as the native executable. The default `web` target embeds the
browser workbench. The `headless` target exposes only the API. Both run as UID/GID
65532, include a health check, accept SIGTERM and require authenticated execution.
The compiler, Node and source tree are left in the build stage.

## Local build and start

From the repository root, create a session token and build:

```sh
export RXCHEF_TOKEN="$(python3 -c 'import secrets; print(secrets.token_hex(32))')"
docker compose build
docker compose up -d
```

Open `http://127.0.0.1:8787`, choose **Set Token**, and use the value from
`RXCHEF_TOKEN`. To display it for your own session, use `printf '%s\n' "$RXCHEF_TOKEN"`.
Keep the same environment when running later Compose commands. A new token
requires reconnecting clients. No default password or token is embedded in the image.

The supplied Compose service uses **host networking** and the server still binds
only `127.0.0.1:8787`. This works on Linux. Docker Desktop 4.34+ requires enabling
**Settings → Resources → Network → Enable host networking**; see
[Docker's host-network documentation](https://docs.docker.com/engine/network/drivers/host/).
Without that opt-in on Desktop, use the native server instead. Do not add `-p`
to host-network mode or assume that container loopback is otherwise host loopback.
The server's existing TLS requirement for non-loopback binding remains intact.

```sh
docker compose ps
docker compose logs --tail 30
docker compose down
```

The service uses a read-only root filesystem, temporary `/tmp`, dropped Linux
capabilities, no-new-privileges, a PID limit and a 2 GiB container memory limit.
These supplement the server's worker limits. They do not enable optional native
operation backends. Port 8787 must be free; stop an existing native server first.

## Standalone images and headless API

```sh
docker build -f docker/Dockerfile.server --target web -t rxchef-server:local .
docker build -f docker/Dockerfile.server --target headless -t rxchef-server:headless .
docker run --rm --network host --read-only --tmpfs /tmp \
  --cap-drop ALL --security-opt no-new-privileges:true \
  -e RXCHEF_TOKEN rxchef-server:headless
```

The examples build locally. No published Docker Hub/GHCR tag is promised yet.
The container CI builds and tests both variants natively on Linux x86_64 and
ARM64 without an image-push step. A public registry publication is a separate release
step after native release acceptance; a successful image build does not approve
untested desktop installers.

## Save an image for another machine

```sh
docker image save -o rxchef-server-local.docker.tar rxchef-server:local
docker image load -i rxchef-server-local.docker.tar
```

Use an image built for the destination architecture. An ARM64 image archive does
not certify native AMD64 execution. Saved image archives contain the runtime;
project exports and input files remain separate backups.

## Remote deployment behind TLS

For a server reachable beyond the local host, provide a real TLS reverse proxy
and use explicit server arguments:

```sh
docker run --rm -p 127.0.0.1:8787:8787 -e RXCHEF_TOKEN rxchef-server:local \
  --listen 0.0.0.0:8787 --allow-remote --trust-tls-terminator \
  --allowed-host chef.example.org --allowed-host 127.0.0.1:8787 \
  --allowed-origin https://chef.example.org
```

Configure the host's TLS proxy to forward to the loopback-published port and
preserve the expected Host. The explicit loopback allowed-host keeps the image
health check working. Replace the example domain with the deployed authority,
including its port if nonstandard. `--trust-tls-terminator` acknowledges an
existing proxy; it does not install TLS. See [configuration](configuration.md).

## Files, persistence and clients

The server accepts selected bytes over the API and does not open project file
references on the host. Its queue/results are temporary and bounded by retention.
Container restart is not a project backup. No project volume is required by this
stateless API.

Browser projects remain in the browser profile at the same origin, independently
of container replacement. Export `.rxproject.json`, recipes/graphs and exact
binary outputs to host files. Desktop uses its own local library; CLI/TUI use
their Store and explicit file exports. See [local storage and backups](../workbench/local-storage.md).
CLI, TUI and desktop can use this server's URL and the same session token.

## Test the built image

```sh
python3 scripts/test-container.py --image rxchef-server:local --variant web
python3 scripts/test-container.py --image rxchef-server:headless --variant headless
```

Add `--host-network` to also check actual host-loopback connectivity when port
8787 is free. On Docker Desktop this requires the opt-in described above.

The default test creates and removes its own container, with no external network or host
ports. It checks non-root/read-only operation, actual health/authentication,
variant-specific assets, exact all-byte multi-input worker outputs, job polling
and clean SIGTERM shutdown. This test does not imply a Docker Desktop host-network
setting was enabled or a remote TLS proxy was deployed.
