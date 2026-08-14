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

## How it works

YARA is a tool developed at VirusTotal, primarily aimed at helping malware researchers to identify and classify malware samples. It matches based on rules specified by the user containing textual or binary patterns and a boolean expression. For help on writing rules, see the <a href='https://yara.readthedocs.io/en/latest/writingrules.html'>YARA documentation.</a>

## Implementation

The implementation is in `src/operations/yara_rules.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "YARA Rules"
```

For file or binary input use `rxchef run "YARA Rules" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "YARA Rules" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/yara_rules.rs

Known-answer:
- none recorded

Differential:
- none recorded

Property:
- none recorded

Fuzz:
- none recorded

## Performance

Not measured. Reason: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
