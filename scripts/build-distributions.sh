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
if [[ -e "$archive" || -L "$archive" || -e "$archive.sha256" || -L "$archive.sha256" ]]; then
  echo "Refusing to overwrite existing archive: $archive" >&2
  exit 1
fi
output=$(mktemp -d "$destination/.stage-XXXXXXXX")
trap 'rm -rf "$output"' EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
mkdir "$output/content"
contents=(LICENSE)
case "$variant" in
  library)
    cargo build --locked --release -p rxchef
    case "$os" in
      Darwin) contents+=(librxchef.a librxchef.dylib librxchef.rlib); cp "$target_dir/release/librxchef.a" "$target_dir/release/librxchef.dylib" "$target_dir/release/librxchef.rlib" "$output/content/" ;;
      Windows) contents+=(rxchef.dll rxchef.lib librxchef.rlib); cp "$target_dir/release/rxchef.dll" "$target_dir/release/rxchef.lib" "$target_dir/release/librxchef.rlib" "$output/content/" ;;
      *) contents+=(librxchef.a librxchef.so librxchef.rlib); cp "$target_dir/release/librxchef.a" "$target_dir/release/librxchef.so" "$target_dir/release/librxchef.rlib" "$output/content/" ;;
    esac ;;
  cli) contents+=("rxchef$suffix"); cargo build --locked --release -p rxchef-cli; cp "$target_dir/release/rxchef$suffix" "$output/content/" ;;
  tui) contents+=("rxchef_tui$suffix"); cargo build --locked --release -p rxchef-tui; cp "$target_dir/release/rxchef_tui$suffix" "$output/content/" ;;
  combined) contents+=("rxchef$suffix" "rxchef_tui$suffix"); cargo build --locked --release -p rxchef-cli -p rxchef-tui; cp "$target_dir/release/rxchef$suffix" "$target_dir/release/rxchef_tui$suffix" "$output/content/" ;;
  headless) contents+=("rxchef-server$suffix"); cargo build --locked --release -p rxchef-server; cp "$target_dir/release/rxchef-server$suffix" "$output/content/" ;;
  web)
    contents+=("rxchef-server$suffix")
    npm --prefix apps/web ci
    npm --prefix apps/web run build
    cargo build --locked --release -p rxchef-server --features web-ui
    cp "$target_dir/release/rxchef-server$suffix" "$output/content/"
    ;;
  desktop) npm --prefix apps/web ci; npm --prefix apps/web run desktop:build; exit ;;
  *) echo 'Usage: build-distributions.sh library|cli|tui|combined|headless|web|desktop' >&2; exit 2 ;;
esac
cp LICENSE "$output/content/"
# The archive is a sibling of content, never part of its own input tree.
# macOS tar must not inject AppleDouble resource-fork sidecars.
COPYFILE_DISABLE=1 tar -czf "$output/archive.tar.gz" -C "$output/content" "${contents[@]}"
python3 - "$output/archive.tar.gz" "$archive" <<'PY'
import hashlib, os, pathlib, signal, sys
archive = pathlib.Path(sys.argv[1])
destination = pathlib.Path(sys.argv[2])
hasher = hashlib.sha256()
with archive.open('rb') as stream:
    for chunk in iter(lambda: stream.read(1024 * 1024), b''):
        hasher.update(chunk)
digest = hasher.hexdigest()
checksum = archive.with_suffix(archive.suffix + '.sha256')
checksum.write_text(f'{digest}  {destination.name}\n')

def interrupted(signum, frame):
    raise InterruptedError(f'publication interrupted by signal {signum}')

for signum in (signal.SIGINT, signal.SIGTERM):
    signal.signal(signum, interrupted)

# Hard links publish complete files atomically and fail if a name already exists.
# Publish the checksum first; the archive is the final commit marker. Roll back
# only our checksum if the archive name races with another producer.
checksum_destination = pathlib.Path(str(destination) + '.sha256')
checksum_created = False
published = False
try:
    os.link(checksum, checksum_destination)
    checksum_created = True
    os.link(archive, destination)
    published = True
finally:
    if checksum_created and not published:
        checksum_destination.unlink()
print(f'{destination}: {archive.stat().st_size} bytes, SHA-256 {digest}')
PY
