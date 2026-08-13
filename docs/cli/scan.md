# Scan command

The `scan` command searches input streams or files for encoded data, suspicious tokens, or other content worth investigating.

## Usage

```bash
rxchef scan dump.bin
rxchef scan ./logs -r --decode
```

## What it helps with

- find candidate strings in large files,
- identify data that may be encoded,
- and optionally hand results to the decode engine for deeper analysis.

The scanner reads in 64 KiB chunks and retains at most `--max-token-size` bytes
per token (default 1 MiB). `--max-findings` stops the complete multi-input scan
after the configured number of results (default 10,000).

## Related pages

- [Magic](magic.md)
- [CLI overview](index.md)
- [Architecture: Scan](../architecture/scan.md)
