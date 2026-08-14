# From Base85

## Overview

Base85 (also called Ascii85) is a notation for encoding arbitrary byte data. It is usually more efficient than Base64.

This operation decodes data from an ASCII string (with an alphabet of your choosing, presets included).

e.g. BOu!rD]j7BEbo7 becomes hello world

Base85 is commonly used in Adobe's PostScript and PDF file formats.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | Available |
| Features | none |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `String`.

## Output

Declared output type: `Bytes`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Alphabet | `String` | no | `!-u` | — | no | The Base85 alphabet |
| 2 | Remove non-alphabet chars | `Boolean` | no | `true` | — | no | Remove characters not in the alphabet before decoding |
| 3 | All-zero group char | `String` | no | `z` | — | no | Character representing an all-zero group (default 'z') |

## How it works

Base85 (also called Ascii85) is a notation for encoding arbitrary byte data. It is usually more efficient than Base64.

This operation decodes data from an ASCII string (with an alphabet of your choosing, presets included).

e.g. BOu!rD]j7BEbo7 becomes hello world

Base85 is commonly used in Adobe's PostScript and PDF file formats.

## Implementation

The implementation is in `src/operations/from_base85.rs` and declares `String` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "From Base85"
```

For file or binary input use `rxchef run "From Base85" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "From Base85" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/from_base85.rs

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
