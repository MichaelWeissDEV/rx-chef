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
| `Chi Square`, `Index of Coincidence` | Both declare `Number` output but returned `to_le_bytes()` — the raw IEEE-754 representation of the score. The runtime's output contract rejected every run ("declared numeric output but produced invalid UTF-8"), so **both operations were unusable through the CLI, recipes and the API alike**. Their tests decoded the result with `f64::from_le_bytes` and asserted only `value >= 0.0`, so they matched the bug rather than catching it. |
| `Generate all hashes` | Reported a **SHA-1 digest under the `SHA0` label** — the source comment read "using SHA1 as proxy". SHA-0 and SHA-1 differ by a rotate in the message schedule, so the published value was simply wrong. The dedicated `SHA0` operation already implemented the real algorithm; its digest function is now shared. |
| `Blowfish Decrypt` | ECB mode did not strip PKCS#7 padding, although `Blowfish Encrypt` applies it and the CBC path in the same file removes it. Decryption returned `secret message\x02\x02`. Upstream rejects unpadded input the same way rx-chef now does. |
| `To Hexdump` | Emitted one space between the hex and ASCII columns where `hexdump -C` and upstream use two, so every line was a character narrower than the reference output. |
| `To Base64` | Its alphabet expansion was a literal substring replacement for the standard alphabet only, so it **rejected `A-Za-z0-9-_`** — an alphabet `From Base64` accepted and upstream supports — with "Must be 64 chars". Encoding and decoding disagreed about which alphabets exist. It also padded unconditionally, emitting `=` for alphabets that contain no padding character. |
| `XXTEA Decrypt` | `to_uint8_array` computed `n - 3` after `n` had already been reduced to 0 for a single-word ciphertext, **panicking on a one-byte input**. |
| `PHP Deserialize` | The serialized byte length was used to slice a `&str`, **panicking** when it landed inside a multi-byte character. Four `0xFF` bytes were enough to reach it. |
| `Disassemble ARM` | Same defect class as `PHP Deserialize`: the hex input was sliced as a `&str` by byte offset. Non-ASCII input becomes three-byte U+FFFD replacement characters, so the length stayed even while the slice landed inside a character and **panicked**. Only reachable with `--features disassembly`, which is why the default-feature sweep did not find it. |
| `LZString Compress` / `Decompress` | Two defects. The bit packer **hardcoded 16 bits per output character**, so three of the four declared formats could not be produced. And both directions iterated Unicode *scalar values* where lz-string works on **UTF-16 code units**: an astral character was truncated (`😀` U+1F600 came back as U+F600) and upstream's own stream for such input could not be decompressed at all. All four formats now work and non-BMP input is byte-identical with upstream. |
| `From Modhex` | Its `Auto` delimiter was accepted by the decoder's match but stripped nothing, so delimited input was rejected as an invalid modhex character. `Auto` is upstream's default; it now keeps only modhex symbols, which let `To Modhex`'s default be aligned to upstream's `Space` without breaking the roundtrip. |
| `src/ffi.rs` | `ffi_boundary` (a `catch_unwind` wrapper) existed but only `rxchef_run` used it. The other **11 `extern "C"` entry points were unguarded**, including `rxchef_magic`, which runs the detection engine over arbitrary caller bytes. Unwinding across `extern "C"` is undefined behaviour. All 12 are now guarded, enforced by a test. |

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

### Required-argument binding

`resolve_named_args` materialised every schema default into a full argument
vector before the runtime saw it. That erased the difference between "not
supplied" and "supplied as the empty string", so `ArgSchema::required` could
never fire: `rxchef run "AES Encrypt"` reached the cipher with a zero-length key
and failed with *"Invalid key length: 0 bytes"* — an error about the value
rather than about the omission. 58 operations declare a required argument with
an empty default, so the flag was decorative across the whole registry.

Binding now runs in `runtime::bind_arguments` over `Vec<Option<String>>`:

```text
1. one None slot per schema argument
2. positional values, left to right
3. named values; unknown names and duplicate assignments rejected
4. required check   -> None + required = MissingArgument
5. defaults         -> None + optional = schema default
6. materialise Vec<String> for the operation
```

`RuntimeError::MissingArgument` is a distinct variant so a frontend can tell
"you forgot this" from "what you gave me is wrong". `validate_operation_args`
keeps the same check for recipe-supplied vectors that simply stop short of the
schema length. No operation declares a required argument *and* a default, and a
test asserts that invariant across all 478 registered operations.

`run`, `pipe` and `recipe` all report the omission identically:

```text
rxchef: operation 'AES Encrypt' requires a value for argument 'Key'
```

### Shared alphabet handling

`To Base64`, `From Base64`, `To Base85` and `From Base85` each carried their
own range expansion and they were not equivalent — which is how the `To Base64`
defect above survived. All four now use `src/alphabet.rs`, a direct port of
upstream's `Utils.expandAlphRange`, which is the single definition every Base-N
operation shares there. Expansion is pure: padding rules and length validation
differ per codec and stay with the caller. Verified against upstream for the
standard, URL-safe and a fully custom alphabet, all three now byte-identical.

### Panic sweep

`tests/tests/panic_sweep.rs` runs every deterministic, side-effect-free
operation against 20 adversarial inputs — empty, single byte, all-high bytes,
invalid UTF-8, truncated multibyte sequences, unbalanced brackets, oversized
numbers — and fails if any of them panics. A panic crosses the FFI boundary as
undefined behaviour, aborts the JSONL server mid-session, and kills the CLI
without an exit code a caller can act on.

The sweep found two reachable panics on first run under default features
(`XXTEA Decrypt` and `PHP Deserialize`) and a third once it was run with
`--all-features` (`Disassemble ARM`) — a reminder that a default-feature sweep
does not cover the optional integrations. All three are fixed and the sweep is
green in both configurations. `src/operations` contains no
`todo!`, `unimplemented!` or `panic!`; the remaining `unwrap()` calls were not
reached by any input in the sweep.

### CLI structure

`crates/cli/src/main.rs` was a single file holding the clap definitions, all
sixteen command implementations, the interactive shell, input selection, output
rendering, and the pipeline step model. It now contains only the doc header,
module declarations, `main`, and the command dispatch, with the rest split
across 20 modules:

```text
crates/cli/src/
├── main.rs        entry point and dispatch only
├── cli.rs         clap parser and every argument group
├── error.rs       CliError, exit codes, human-readable rendering
├── input.rs       literal / file / stdin selection
├── output.rs      raw, hex and JSON writing
├── steps.rs       pipeline step model, execution, history
├── shell/         interactive shell (mod.rs + completion.rs)
└── commands/      one module per subcommand
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

### Sweep triage

`verification/differential-triage.json` records a verdict and a checkable
reason for every operation in the upstream default sweep — both the output
differences and the rx-chef errors, which were previously summarised only as a
count. Four tests enforce that it stays complete: every entry must carry a
known classification and a reason of substance, no entry may read `UNTRIAGED`,
every `RXCHEF_BUG` must be marked fixed, and operations may not repeat.

```text
EXACT                        168
NOT_COMPARABLE               249
VALID_SEMANTIC_DIFFERENCE     17
NONDETERMINISTIC              15
RXCHEF_BUG                    10   (all fixed; a test enforces this)
ARGUMENT_MAPPING_DIFFERENCE    6
REFERENCE_CAPTURE_BUG          6
DEFAULT_MAPPING_DIFFERENCE     3
UNSUPPORTED_UPSTREAM_MODE      3
```

Three findings are worth naming because they change what the earlier summary
implied:

* Six differences were **defects in the capture harness, not in rx-chef**.
  `Unicode Text Format` is the only upstream operation declaring a boolean
  argument with the *string* value `"false"`, which is truthy in JavaScript, so
  the sweep made upstream apply formatting that rx-chef correctly omitted.
  Five more returned structured objects or browser types the sweep stringified
  to `[object Object]` / `[object File]`.
* Most `NOT_COMPARABLE` entries are the sweep input `foobar` not being valid
  input for a decoder. Rejecting it is correct behaviour, not a divergence.
* `Gzip`'s upstream output embeds a modification timestamp and is **not
  reproducible between captures**, which is what motivated the semantic
  comparison below.

### Compression parity

DEFLATE and bzip2 do not specify a unique encoding for a given input, so two
conforming encoders may legitimately disagree byte for byte. Demanding exact
equality there would either force a false MISMATCH or invite pasting rx-chef's
own output into the fixture, which proves nothing.

The harness therefore has a `semantic_roundtrip` expectation: both rx-chef's
output *and* the recorded upstream stream must decode back to the original
input through rx-chef's inverse operation. That establishes interoperability in
both directions without asserting equality the format never promised. It
reports `EXACT` when the streams happen to be identical and `COMPATIBLE`
otherwise. `Gzip`, `Zlib Deflate` and `Raw Deflate` currently report
`COMPATIBLE`; rx-chef reads every upstream stream correctly.

### Reference capture

`tools/cyberchef-reference/capture.mjs` runs operations in an upstream CyberChef
checkout and records their output, so fixture values are *observed* rather than
recalled. It is a developer tool: the release gate compares rx-chef against the
recorded fixture and needs neither Node nor a CyberChef checkout.

```bash
CYBERCHEF_DIR=/path/to/CyberChef node tools/cyberchef-reference/capture.mjs requests.json
```

Every capture records the upstream **git commit**, not just the package
version: "CyberChef 11.0.0" spans every commit between two releases and cannot
identify what was observed. The tool refuses to run when the checkout has
uncommitted changes to operation sources, since the recorded commit would then
not describe what ran, and records any other local modifications so a reviewer
can judge them. The fixture carries the same provenance block.

The values in this repository were observed with
`CyberChef 11.0.0 @ 0bb5472e50e158ee1885aab02a2ce93adf538656`
(`v11.0.0-18-g0bb5472e`); the capture checkout had local changes to a lockfile
and scratch files only, with operation sources clean.

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

### The correctness dimension is derived, not declared

`implementation_status` is a trait method that returned `Partial` for **all 478**
operations — not as a judgement, but because it is the trait default and nobody
ever set it. 473 of them carried a benchmark skip reason reading *"operation
remains Partial until performance evidence is reviewed"*, conflating how fast an
operation is with whether it is correct.

Correctness is now computed by the audit from evidence in the repository:

```text
unverified          no executable tests
partially_verified  tests exist, but negative / boundary / independent
                    reference evidence is incomplete
verified            executable tests covering normal, negative and boundary
                    input, plus at least one independent reference
                    (known-answer or differential)
```

A verdict that can be raised by editing a line is not evidence, so this one can
only be raised by adding tests. Each operation also carries a
`correctness_gaps` list naming exactly what it still needs, which is what makes
the remaining work enumerable instead of estimated.

### Progress

The current figures are in `docs/_generated/verification-backlog.md`, which
also lists every remaining operation with its exact gap.

### Bulk differential capture

The single largest gap was independent reference evidence. Rather than writing
fixtures by hand, every loadable upstream operation was run against five
standard probes — normal text, empty, UTF-8, binary and a single byte — under
each side's own defaults, and rx-chef was run on the same inputs. Only the
pairs that agreed were recorded, since an agreement is the evidence:

```text
1363  upstream outputs captured across 475 operations
 845  agreeing rx-chef/upstream pairs across 223 operations
 672  new fixture cases (the rest already had one)
```

Differential evidence grew substantially; the current case and verdict counts
come from the differential test's own summary output. The 232 disagreements and 204 rx-chef errors from that sweep are
covered by the triage below rather than left as a number.



From `cargo run -p xtask -- audit-operations`:

<!-- Numbers are not repeated here. They go stale the moment a test lands.
     The single source of truth is the generated audit artefact. -->

See **`docs/_generated/operation-quality.json`** (`summary` object) and
**`docs/_generated/verification-backlog.md`** for the current figures, both
regenerated by:

```bash
cargo run -p xtask -- audit-operations
```


Differential fixture verdict counts are printed by `cargo test -p cyberchef-rust-tests --test differential -- --nocapture`.

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

**Last container run**: every gate passed except `all-features tests`, which
did not fail on a test — the linker was OOM-killed:

```text
collect2: fatal error: ld terminated with signal 9 [Killed]
```

Docker Desktop on the development host allocates 7.65 GiB, and linking the
all-features build (1891 crates in the dependency graph, against 1461 for the
default set) exceeds it. The same suite passes natively on the host: 2101 tests
across 16 targets, `cargo test --workspace --all-features`, zero failures. This
is an environment limit, not a code defect; raising the container's memory
allocation is the fix, and the run is pending.

Everything else in the container passed, including all-features **check**,
both clippy passes, the FFI C compile/link/run, `cargo package`, `cargo install`
and the installed-CLI smoke test.

The hardcoded `jq -e "length == 478"` operation-count check was replaced with a
count derived from the audit output.

All-features clippy is now part of the gate. The default feature set leaves the
optional integrations (PGP, JSONata, OCR, YARA, disassembly) unlinted, so
without it that code was never checked at all:

```bash
cargo clippy --locked --workspace --all-targets --all-features -- \
    -D clippy::correctness -D clippy::suspicious
```

### CI trigger

`.github/workflows/platform-checks.yml` triggered on pushes to `main`, but the
repository develops on `master` — so **no push to the default branch had ever
started these checks**. The trigger now lists both branches, keeping the
workflow working if the branch is ever renamed:

```yaml
push:
  branches:
    - master
    - main
```

## Open items

**P0** — none known.

**P1**

* **The operations audit is incomplete.** `docs/_generated/verification-backlog.md`
  is generated per run and lists every operation that is not yet `verified`,
  grouped by what it needs and with a recommended strategy. That file, not a
  number copied into prose, is the working list.
* 301 operations have unverified CyberChef parity. The `parity()` metadata is
  a separate, stricter claim than differential evidence: it is only raised for
  operations with a recorded differential case *and* a reviewed decision.
* Nine operations carry an unresolved mapping difference, each with a stated
  cause in `verification/differential-triage.json`; none is a confirmed defect.
* The remote GitHub Actions run has **not been verified** — no authenticated
  `gh` session was available.
* The all-features gate in the Linux container is **pending**: the linker is
  OOM-killed at Docker's 7.65 GiB allocation. The same suite passes natively.

**P2**

* `LZString Compress` implements all four formats. Two limitations remain
  recorded in `known_limitations()`: the Standard format emits raw UTF-16 code
  units, so astral input yields lone surrogates that UTF-8 cannot hold and is
  reported as an error (the other three formats are unaffected); and
  `EncodedURIComponent` has no upstream counterpart, so it has no parity
  reference.
All six Base-N operations now share `src/alphabet.rs`. `from_base32`'s local
copy also special-cased the two standard alphabets — both of which the general
path produces identically — and reached `char::from_u32(..).unwrap()`, which
panics on surrogate code points from a caller-supplied range.
* The Linux container gate has **not been run in this round** — Docker was
  unavailable on the development host. See below.
* `cli.rs` still holds every argument group in one module. Splitting it per
  command group is possible but was not needed to remove the monolith.

## Not run

| Check | Reason | Still to verify |
|---|---|---|
| Remote GitHub Actions execution | No authenticated `gh` session was available on the development host (`gh auth status`: not logged in). | That the corrected `push` trigger actually starts the three jobs on `master` and that they pass. The trigger itself was verified by parsing the workflow: `push.branches` is now `[master, main]`. |
| Native macOS / Windows matrix jobs | Runnable only on those runners; the local host is `Darwin arm64`. | `scripts/check-native-platform.sh` on `macos-latest` and `windows-latest`. |

The Linux release container **was** run in this round and is the reference
result recorded above.
