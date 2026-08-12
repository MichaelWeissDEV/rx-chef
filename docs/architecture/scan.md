# Scan engine

The scan engine is designed for large or unknown input streams where the user may need to locate encoded data, suspicious tokens, or high-entropy blobs without needing to inspect the file manually.

Instead of assuming the whole payload is already known, the scanner walks through input in chunks and identifies candidate strings that look like encoded or structured values.

## Purpose

This is useful for situations like:

- binary dumps,
- memory captures,
- log files,
- network captures,
- or large generated data streams.

The idea is to find potentially interesting chunks first and then decide whether they should be decoded or reported.

## Main characteristics

- works on files, directories, and streams,
- handles large inputs incrementally,
- classifies candidate tokens,
- and can trigger the decode workflow for promising results.

## Integration with magic

The scan engine can hand promising detections to the magic engine for recursive decode attempts. This allows the system to be both broad in discovery and deep in interpretation.

## Related pages

- [Magic engine](magic.md)
- [Architecture overview](overview.md)
- [Scan CLI](../cli/scan.md)
