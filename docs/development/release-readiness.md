# Release readiness

Status of the quality hardening round. Numbers here come from
`cargo run -p xtask -- audit-operations`, which derives them from the
repository rather than from hand-maintained records.

## Starting state

The workspace already built and tested cleanly. `cargo fmt`, `cargo check`
(default and `--all-features`), `cargo test --workspace`, `cargo clippy` with
`-D clippy::correctness -D clippy::suspicious`, and the `xtask` registry, audit
and documentation gates all passed at `40bb955`.

The problem was not a broken build; it was that several of the signals used to
judge quality were not measuring what they claimed to.

## Problems found and fixed

### The operation audit reported evidence it had not checked

`xtask audit-operations` set `correctness_test: true` for an operation whenever
a file named after it existed under `tests/tests/operations/`. It never read
the file. **37 registered operations had a test file containing zero `#[test]`
functions** and were still counted as tested.

`crates/xtask/src/test_evidence.rs` now parses the mapped sources and reports
what is actually in them: `#[test]` count, negative-assertion count,
empty-input and boundary coverage, exact-value assertions, and bare
`assert!(x.is_ok())` assertions. An operation whose evidence contains no test
must now carry an explicit `untested_reason` in
`verification/operations.json`, or the audit fails.

The audit also refuses to accept parity claims without matching evidence:
`Compatible` requires a known-answer vector and `Exact` requires a differential
case, so a specification vector can no longer be mistaken for CyberChef parity.

### Correctness bugs

| Operation | Defect |
|---|---|
| `HMAC` | The `Key` argument is declared `ArgKind::Bytes`, but the operation read it with `as_str()`. Every caller passing `hex:`/`0x`/`base64:` — the documented spellings — reached the operation as `ArgValue::Bytes`, `as_str()` returned `None`, and the code **silently fell back to an empty key**, producing an authentication code keyed with nothing. Now uses the shared `Utils::convert_to_byte_array`. |
| `To Base32` | Passed its alphabet argument straight to `data_encoding`, which does not understand range notation. The operation **rejected its own declared default** (`A-Z2-7=`) and could not run at all. Now shares `expand_base32_alphabet` with `From Base32`. |
| `LZString Compress` | Pushed each 16-bit code unit through `char::from_u32(..).unwrap()`, **panicking** on the UTF-16 surrogate range the bit stream can produce. Now returns a structured error. |
| `LZString Compress` | Ignored its `Compression Format` argument entirely (`bits_per_char` was hardcoded), so requesting Base64 returned a standard-format stream. Unimplemented formats are now reported. |
| `LZString Compress` / `Decompress` | Declared disjoint format vocabularies (`default` vs `Standard`), so the two defaults could never be paired. Now share one enum. |
| `URL Encode` | Emitted **lowercase** percent-escapes (`%c3%a9`). ECMA-262's `Encode` operation — which `encodeURI`/`encodeURIComponent`, and so upstream CyberChef, are built on — always emits uppercase, as does RFC 3986 §2.1. Every escape differed from upstream. |
| `GOST Hash` | An unrecognised `Algorithm` fell through to the Streebog branch, **silently returning a digest from a different hash function**. Now rejected; `Algorithm` and `sBox` are proper enums. |
| `To Table` | ASCII and Markdown borders were one column narrower than the cells, and widths were measured in UTF-8 bytes, so any multibyte cell skewed the row. |
| `IPv6 Transition Addresses` | Returned the prose "Enter compressed or expanded IPv6 address…" as a **successful result**, so the CLI exited 0 and the message could be mistaken for a converted address. Now `InvalidInput`. |
| `From Morse Code` | **Decoded every input to an empty string.** Its signal normalisation chained `str::replace`, and five of the patterns were the *empty string* — `"ab".replace("", "x")` yields `"xaxbx"` in Rust, so a token was inserted between every character and no signal could match the lookup table. The four tests covering it asserted only `result.is_ok()` with the comment "test that the operation runs without panicking", so a completely broken operation looked tested. Normalisation now mirrors upstream's two case-insensitive regex passes. |
| `To Punycode` / `From Punycode` | `To Punycode` went through `domain_to_ascii` and stripped the `xn--` prefix. For all-ASCII input that returns the string unchanged, so no delimiter was emitted: `"foobar"` encoded to `"foobar"`, which `From Punycode` then decoded to `䕮䕮䕭䕲`. **The encoding was not round-trippable.** RFC 3492 §6.3 requires the trailing delimiter (`foobar-`), which Python's own punycode codec and CyberChef both produce. Both directions now use `idna::punycode` directly. |
| `SHA3`, `Keccak`, `RIPEMD`, `Streebog` | Default digest sizes differed from upstream (SHA3/Keccak 256 vs 512, RIPEMD 160 vs 320, Streebog 512 vs 256), so `rxchef run SHA3` and CyberChef's SHA3 with default settings returned **different digests of different lengths**. The algorithms were correct — only the defaults diverged. Now aligned; verified against observed upstream output. |

### Drift

`docs/_generated/operation-status.json` lived in a `_generated` directory but
was hand-maintained, read by nothing, and stale: it recorded version `0.0.1`
against a `0.1.0` workspace and omitted the `yara` and `disassembly` features
along with three feature-gated operations. It is now generated from the live
workspace by `cargo xtask verify-version`, which is a release gate.

### Persistence

Store files were already written `0600` and variables already support a
`secret` flag with redaction. Directories, however, were created with
`create_dir_all` at the umask default (commonly `0755`), leaving the *names* of
stored recipes and projects world-readable. `ensure_dir` now applies `0700` to
directories **it creates**; store directories created by earlier versions keep
their existing mode and are not retrofitted.

### CLI structure

`crates/cli/src/main.rs` was a 2655-line file holding the clap definitions, all
sixteen command implementations, the interactive shell, input selection, output
rendering, and the pipeline step model. It is now 119 lines — the doc header,
module declarations, `main`, and the command dispatch — with the rest split
across 20 modules:

```text
crates/cli/src/
├── main.rs        119   entry point and dispatch only
├── cli.rs         630   clap parser and every argument group
├── error.rs        60   CliError, exit codes, human-readable rendering
├── input.rs        84   literal / file / stdin selection
├── output.rs      204   raw, hex and JSON writing
├── steps.rs       252   pipeline step model, execution, history
├── shell/               interactive shell (mod.rs + completion.rs)
└── commands/            one module per subcommand
```

The split is behaviour-preserving, and that was checked rather than assumed: the
pre-refactor binary was built from `HEAD` in a git worktree and compared against
the new one. `--help` for all 18 subcommands, `--version`, and the generated
manpage are byte-identical; `operations --all --json` differs only in the
`parity`, `deterministic`, `side_effects`, `known_limitations` and `args` fields
changed by the metadata work, with the same 478 operation names. The 7 unit
tests that lived in `main.rs` moved with their subjects (5 to `steps.rs`, 2 to
`input.rs`) and the 46 integration tests were untouched.

### Argument semantics — no duplicate parsers in the frontends

Checked directly rather than assumed. The CLI, TUI, and FFI all execute through
`execution::execute`, which calls `runtime::validate_operation_args` and
`runtime::run_operation`; `src/ffi.rs` normalises its typed `ArgValue` into the
same `num:` / `bool:` / `hex:` spellings the runtime parses. No frontend carries
its own parser or validator.

The one genuine violation was inside an operation, not a frontend: `HMAC`
carried a private `decode_key` duplicating the shared byte-argument semantics
(and diverging from them). It was removed, and its `0x` spelling folded into
`Utils::convert_to_byte_array` so every byte argument accepts it.

### Error-as-output sweep

`IPv6 Transition Addresses` returned prose as a successful result. A sweep of
`src/operations/` for the same shape found no other instance. The remaining
message-returning operations — `Detect File Type` ("Unknown file type."),
`Untar`/`Unzip` ("No files found…"), `ECDSA Verify`/`RSA Verify`
("Verification Failure") — return legitimate *results*, not errors, and match
upstream behaviour.

## Test improvements

* `tests/tests/known_answer_vectors.rs` — 17 tests built from RFC 4648
  (Base16/32/64), RFC 1321 (MD5), FIPS 180-4 (SHA-1, SHA-2), FIPS 202 (SHA-3),
  RFC 2202 and RFC 4231 (HMAC), RFC 7693 (BLAKE2), RFC 1950 (Adler-32) and
  NIST SP 800-38A (AES-128 ECB/CBC/CTR/CFB/OFB). Every expected value is quoted
  from the standard; none was generated by running rx-chef.
* `tests/tests/differential.rs` plus `tests/fixtures/differential/cases.json` —
  the CyberChef parity harness (see below).
* New test modules for 14 previously untested operations, several of which
  found the bugs listed above.
* `tests/tests/operations/hmac.rs` — `test_hmac_sha1_rfc2202_case1` used a
  16-byte key while claiming to be RFC 2202 case 1, which uses 20 bytes. Its
  expected digest was therefore implementation-derived, not a reference vector.
  Corrected, and regression tests added for the byte-key bug.

## Differential harness

```text
tests/fixtures/differential/cases.json
       |
       +--> recorded CyberChef reference output
       |
       +--> rx-chef output for the same operation/input/args
                  |
             normalize both  (line endings only)
                  |
               compare -> EXACT | DOCUMENTED_DIFFERENCE | MISMATCH
                          | NOT_COMPARABLE | UNVERIFIED
```

Normalization is deliberately minimal — CRLF/CR to LF and one trailing newline
— because anything more can hide a real behavioural difference. Operations that
declare non-determinism or side effects must be marked `not_comparable`; a test
enforces this against the operation metadata rather than a hand-kept list.

Every case must carry a `reference_source` a reviewer can check — either a
published specification CyberChef also implements, or a value observed directly
from upstream. The harness fails if one is missing. This exists because the
first draft of the fixture contained a `URL Encode` case whose expected value
was reconstructed from recollection of upstream's `encodeAllChars` rather than
observed; it is now marked `unverified`, and `url_encode` was downgraded from
`Exact` to `Compatible` accordingly. Its lowercase-to-uppercase fix stands on
RFC 3986 §2.1 and ECMA-262, which is a specification claim, not a parity claim.

Current fixture: **180 EXACT, 1 DOCUMENTED_DIFFERENCE, 2 NOT_COMPARABLE**.

On a MISMATCH the expected value must not simply be edited. Establish first
whether rx-chef is wrong, CyberChef differs, the normalization is wrong, or the
difference is deliberate.

### Reference capture

`tools/cyberchef-reference/capture.mjs` runs operations in an upstream CyberChef
checkout and records their output, so fixture values are *observed* rather than
recalled. It is a developer tool: the release gate compares rx-chef against the
recorded fixture and needs neither Node nor a CyberChef checkout.

```bash
CYBERCHEF_DIR=/path/to/CyberChef node tools/cyberchef-reference/capture.mjs requests.json
```

171 of the current cases were produced by a sweep that ran every loadable
upstream operation and every rx-chef operation against the same input under each
side's own defaults, then kept only the pairs that agreed. That sweep is also
what surfaced the four diverging hash defaults above.

Sweep result over 283 comparable upstream operations: 167 already agreed, 44
errored on rx-chef's side, 18 were excluded as non-deterministic, and 54
differed. Most of those 54 are artifacts of resolving upstream's nested option
groups to a single default, not defects; each was triaged individually rather
than assumed. The confirmed defects are listed above, and the untriaged
remainder is tracked under Open items.

The earlier `URL Encode` case that rested on recollection is now marked `exact`
against an observed value, and `url_encode` is back to `Exact` parity.

## Operation quality

From `cargo run -p xtask -- audit-operations`:

```text
registered                   478
with executable tests        456
test functions total        1886
with negative tests          195
with boundary tests          251
reference-verified (KAT)      15
differential-verified        173
parity exact                 171
parity compatible              5
parity documented difference   1
parity unverified            301
documented divergences         1
tests asserting only success  69
```

The 22 operations without executable tests each carry a reviewed
`untested_reason` in `verification/operations.json` — network I/O, feature-gated
PGP, or a missing capture/certificate fixture.

`parity unverified` at 301 is the honest state: those operations have not been
compared against CyberChef. It is a measurement, not a regression. The audit
enforces the distinction rather than trusting it — `Exact` requires a recorded
differential case, `Compatible` requires known-answer evidence, and a
known-answer claim requires the mapped test to assert an exact value.

`tests asserting only success` counts test *functions* whose body asserts
`is_ok()` but never an exact value. It fell from 86 to 69 as the encoding and
Morse families were rewritten against observed upstream values; the remainder is
tracked under Open items.

## Release gate

`scripts/release-gates.sh` is the single definition of the gate set, run both
by the Linux container (`docker/release-entrypoint.sh`) and directly on a host:

```bash
scripts/release-check-linux.sh          # authoritative, Linux x86_64 container
scripts/release-check-linux.sh --host   # portable subset on this machine
```

Both print `RX-CHEF RELEASE CHECK: PASS` only when every gate passed. Host mode
lists the Linux-only gates it skipped rather than dropping them silently. Every
gate runs even after one fails, so the summary reports all failures at once.

The hardcoded `jq -e "length == 478"` operation-count check was replaced with a
count derived from the audit output.

## Open items

**P0** — none known.

**P1**

* 301 operations have unverified CyberChef parity. The harness and the capture
  tool both exist; the fixture needs cases for the operations the default sweep
  could not compare.
* 54 operations differed in the upstream default sweep and are not yet fully
  triaged. The confirmed defects among them were fixed; the rest are believed to
  be artifacts of resolving upstream's nested option groups to a single default,
  but that has not been established case by case. Notable untriaged candidates:
  `To Hexdump` (padding width), `To Modhex` (delimiter), `Entropy` (float
  precision), `Rison Decode` / `YAML to JSON` (scalar quoting), `Render Markdown`
  (`<p>` wrapper), `ROT13 Brute Force` (label format), `Generate HOTP` (secret
  encoding), and the compression operations (`Gzip`, `Raw Deflate`,
  `Zlib Deflate`, `LZ4`, `LZMA`), whose streams differ but decompress correctly.
* 69 test functions assert only that an operation succeeded, without checking
  the value. The audit counts them per test function — a redundant
  `assert!(result.is_ok())` followed by an `assert_eq!` is not counted — so the
  number can be driven down deliberately. The largest remaining groups are
  `Generate all checksums`/`Generate all hashes` (5 each), `Blowfish Encrypt`
  (5), `Bzip2 Compress`, `Cartesian Product`, `ECDSA Signature Conversion` and
  `ELF Info` (4 each).
* `required: true` is not enforced centrally. 58 operations declare a required
  argument with an empty default; every frontend fills defaults before calling
  `runtime::run_operation`, so the runtime never sees a missing value and the
  operation itself has to catch it (`AES Encrypt` reports an invalid key
  *length* rather than a missing key). The flag is currently decorative.

**P2**

* `LZString Compress` implements only the `Standard` output format; Base64,
  UTF16 and EncodedURIComponent are declared and rejected rather than produced.
* Alphabet expansion is duplicated across `to_base64`, `from_base64`,
  `to_base85`, `from_base85` and `from_base32`. Only the Base32 pair was
  unified here.
* The Linux container gate has **not been run in this round** — Docker was
  unavailable on the development host. See below.
* The CLI split raised the crate from one file to 21; `cli.rs` (630 lines) still
  holds every argument group. Splitting it per command group is possible but was
  not needed to remove the monolith.

## Not run

| Check | Reason | Still to verify |
|---|---|---|
| `scripts/release-check-linux.sh` (container) | Docker was not running on the development host (`docker info` failed). | FFI C linking (`cc -lrxchef`, `LD_LIBRARY_PATH`), `cargo package`, `cargo install`, quick Linux benchmarks, and `mkdocs build --strict`. |
| `mkdocs build --strict` | `mkdocs` is not installed on the development host; the host gate reports it as skipped. | Documentation build after the operation-doc regeneration. |

Everything else in this document was verified by running it on
`Darwin arm64` with `rustc 1.97.1`.
