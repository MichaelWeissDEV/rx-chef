# Security model

rxchef transforms untrusted bytes in the local process. Most operations are
pure, but some parse complex formats, decompress archives, access files, make
HTTP requests, or invoke optional native libraries. Treat recipes and project
files from another person as code-like input and inspect their operations first.

Important boundaries:

- stdin/stdout may contain secrets; shell history also records literal
  `--input`, arguments, and `--set` values;
- saved variables and history are local convenience storage, not a secret vault;
- decompression and archive operations can expand small inputs dramatically;
- `HTTP request`, filesystem/project input, and similar operations can cross the
  local data boundary;
- cryptographic operations expose primitives and do not design a secure protocol
  for the caller;
- operations marked `broken` by `operations --json` must not be used for results.

For automation, use exact operation names, bounded input sizes, timeouts around
the process, and `--json`/the stdio protocol rather than scraping human output.
