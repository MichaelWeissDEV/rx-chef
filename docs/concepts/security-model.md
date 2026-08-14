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
- randomness-producing operations use their documented backend; deterministic
  pseudorandom generators must not be substituted for cryptographic randomness;
- optional OCR and other native libraries extend the parser/ABI attack surface;
- History stores only bounded previews and metadata, and redacts every argument
  marked `sensitive`; previews are never replay input;
- `serve --stdio` is a local child-process protocol, not an authenticated network
  service; request lines are bounded and stdout is JSONL-only;
- the experimental C ABI catches Rust panics, validates pointer/length pairs,
  and requires callers to free each returned allocation exactly once;
- operations whose `availability` is not `available` cannot execute in that build.

For automation, use exact operation names, bounded input sizes, timeouts around
the process, and `--json`/the stdio protocol rather than scraping human output.
