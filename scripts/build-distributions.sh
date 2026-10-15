#!/usr/bin/env bash
# Local packaging only: never publishes, pushes or signs.
set -euo pipefail
cd "$(dirname "$0")/.."
variant=${1:-web}
target_dir=${CARGO_TARGET_DIR:-target}
version=$(cargo metadata --locked --format-version 1 --no-deps | python3 -c 'import json,sys; print(next(p["version"] for p in json.load(sys.stdin)["packages"] if p["name"] == "rxchef"))')
os=$(uname -s)
suffix=
case "$os" in MINGW*|MSYS*|CYGWIN*) os=Windows; suffix=.exe ;; esac
artifact="rxchef-$version-$variant-$os-$(uname -m)"
destination=${RXCHEF_DISTRIBUTION_DIR:-"$target_dir/distributions"}
mkdir -p "$destination"
archive="$destination/$artifact.tar.gz"
if [[ -e "$archive" ]]; then
  echo "Refusing to overwrite existing archive: $archive" >&2
  exit 1
fi
output=$(mktemp -d "$destination/.stage-XXXXXXXX")
trap 'rm -rf "$output"' EXIT
case "$variant" in
  library)
    cargo build --locked --release -p rxchef
    case "$os" in
      Darwin) cp "$target_dir/release/librxchef.a" "$target_dir/release/librxchef.dylib" "$target_dir/release/librxchef.rlib" "$output/" ;;
      Windows) cp "$target_dir/release/rxchef.dll" "$target_dir/release/rxchef.lib" "$target_dir/release/librxchef.rlib" "$output/" ;;
      *) cp "$target_dir/release/librxchef.a" "$target_dir/release/librxchef.so" "$target_dir/release/librxchef.rlib" "$output/" ;;
    esac ;;
  cli) cargo build --locked --release -p rxchef-cli; cp "$target_dir/release/rxchef$suffix" "$output/" ;;
  tui) cargo build --locked --release -p rxchef-tui; cp "$target_dir/release/rxchef_tui$suffix" "$output/" ;;
  combined) cargo build --locked --release -p rxchef-cli -p rxchef-tui; cp "$target_dir/release/rxchef$suffix" "$target_dir/release/rxchef_tui$suffix" "$output/" ;;
  headless) cargo build --locked --release -p rxchef-server; cp "$target_dir/release/rxchef-server$suffix" "$output/" ;;
  web)
    npm --prefix apps/web ci
    npm --prefix apps/web run build
    cargo build --locked --release -p rxchef-server --features web-ui
    cp "$target_dir/release/rxchef-server$suffix" "$output/"
    ;;
  desktop) npm --prefix apps/web ci; npm --prefix apps/web run desktop:build; exit ;;
  *) echo 'Usage: build-distributions.sh library|cli|tui|combined|headless|web|desktop' >&2; exit 2 ;;
esac
cp LICENSE "$output/"
tar -czf "$output/archive.tar.gz" -C "$output" --exclude=archive.tar.gz .
python3 - "$output/archive.tar.gz" "$archive" <<'PY'
import hashlib, pathlib, sys
archive = pathlib.Path(sys.argv[1])
destination = pathlib.Path(sys.argv[2])
hasher = hashlib.sha256()
with archive.open('rb') as stream:
    for chunk in iter(lambda: stream.read(1024 * 1024), b''):
        hasher.update(chunk)
digest = hasher.hexdigest()
archive.with_suffix(archive.suffix + '.sha256').write_text(f'{digest}  {destination.name}\n')
print(f'{destination}: {archive.stat().st_size} bytes, SHA-256 {digest}')
PY
ln "$output/archive.tar.gz" "$archive"
ln "$output/archive.tar.gz.sha256" "$archive.sha256"
