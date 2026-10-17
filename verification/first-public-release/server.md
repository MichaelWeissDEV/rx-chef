# Server acceptance evidence

Date: 2026-10-17 (Europe/Berlin). Host: macOS 26.4 (25E246), arm64.
Candidate version during the full integration run: `0.2.0-rc.1`.

## Results

- `cargo test --locked -p rxchef-server --test server`: **25 passed, 0 failed**, 15.42 seconds.
- Queue/concurrency regression: **40 passed**, four simultaneous test processes.
- `cargo fmt --all --check`: passed.
- `git diff --check`: passed.
- `RXCHEF_TOKEN=local-compose-validation-0123456789 docker compose config --quiet`: passed.

The full integration suite used real loopback listeners and supervised worker processes. It covered authentication and read scopes, Host/Origin rejection, body/execution bounds, queue bounds, cancellation, deadlines, shutdown, WebSocket limits/flood cleanup, project validation and exact binary graph/remote output equivalence.

The queue test previously assumed that submission order guaranteed semaphore acquisition order after a fixed 500 ms sleep. Async tasks can be scheduled in either order. The test now waits for the first job's observable Running state before submitting the second job. Related cancellation and shutdown tests also wait for Running instead of assuming startup completed after a fixed sleep. No production worker state machine change was needed.

## Repetition command

Executed from the repository root after compiling the focused test:

```sh
cargo test --locked -p rxchef-server --test server queue_and_concurrency_are_bounded -- --exact
python3 - <<'PY'
import concurrent.futures, pathlib, subprocess
binaries = list(pathlib.Path('target/debug/deps').glob('server-*'))
binary = max((p for p in binaries if p.is_file() and p.suffix == ''), key=lambda p: p.stat().st_mtime)
def check(n):
    run = subprocess.run([str(binary), '--exact', 'queue_and_concurrency_are_bounded'], capture_output=True, text=True, timeout=60)
    if run.returncode:
        raise RuntimeError(f'iteration {n}: {run.stdout}\n{run.stderr}')
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as executor:
    list(executor.map(check, range(40)))
print('PASS: 40 queue/concurrency regressions, four simultaneous test processes')
PY
```

The full suite and successful repetition run required the execution sandbox's local-socket restriction to be lifted. Earlier sandboxed attempts failed at `TcpListener::bind` with `EPERM`; these were environmental failures and are not counted as passing runs. The initial single focused run passed before that restriction affected subsequent invocations.

## Configuration and container review

Inspected `crates/server/src/config.rs`, HTTP guard/static serving, `crates/server/build.rs`, shared worker manager, Dockerfile, Compose and container smoke workflow. Defaults bind loopback, require bearer authentication for execution/discovery, check Host and Origin, bound queue/workers/results, and deny host effects. Embedded assets are feature-gated. The supplied runtime image uses an unprivileged user; Compose uses read-only rootfs, dropped capabilities and no-new-privileges. No concrete release blocker was identified in these inspected paths. This was a focused release review, not an exhaustive security audit.

Actual Docker image build/runtime acceptance has **not passed locally**. Docker CLI exists, but `docker info --format '{{.ServerVersion}} {{.Architecture}}'` reports that the daemon socket is unavailable. Docker Desktop was launched in the background using `cua-driver launch_app '{"bundle_id":"com.docker.docker"}'`; subsequent checks still showed no daemon. No system settings or host networking opt-ins were changed. Existing CI defines web/headless x64/arm64 image builds and runtime smoke tests, but configuration inspection is not evidence those remote jobs passed.

## Native GUI acceptance

Pending a freshly built production candidate. Background GUI automation is ready: installed cua-driver daemon, Accessibility and Screen Recording granted. No stale/debug RX-Chef bundle was used for acceptance.
