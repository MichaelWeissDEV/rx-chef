# YARA Rules

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

## Overview

YARA is a tool developed at VirusTotal, primarily aimed at helping malware researchers to identify and classify malware samples. It matches based on rules specified by the user containing textual or binary patterns and a boolean expression. For help on writing rules, see the <a href='https://yara.readthedocs.io/en/latest/writingrules.html'>YARA documentation.</a>

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | FeatureDisabled |
| Input requirement | `Required` |
| Features | yara |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `Bytes`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Rules | `String` | no | `<empty>` | — | no | YARA rules to match against the input |
| 2 | Show strings | `Boolean` | no | `false` | — | no | Display the strings that matched |
| 3 | Show string lengths | `Boolean` | no | `false` | — | no | Display the lengths of the strings that matched |
| 4 | Show metadata | `Boolean` | no | `false` | — | no | Display the metadata for each rule |
| 5 | Show counts | `Boolean` | no | `true` | — | no | Display the number of times each rule matched |
| 6 | Show rule warnings | `Boolean` | no | `true` | — | no | Display any warnings generated during rule compilation |
| 7 | Show console module messages | `Boolean` | no | `true` | — | no | Display any messages from the console module |

## Implementation

The implementation is in `src/operations/yara_rules.rs` and declares `Bytes` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "YARA Rules" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `Bytes` value is valid. Its `String` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/yara_rules.rs

Known-answer tests:
- tests/tests/operations/yara_rules.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
