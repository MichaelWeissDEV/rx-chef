# Optical Character Recognition

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

## Overview

Optical character recognition or optical character reader (OCR) is the mechanical or electronic
        conversion of images of typed, handwritten or printed text into machine-encoded text.


        Supported image formats: png, jpg, bmp, pbm.


        Requires Tesseract library. Enable with: --features tesseract

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | FeatureDisabled |
| Features | tesseract |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `Bytes`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Show confidence | `Boolean` | no | `true` | — | no | Whether to show the confidence level of the OCR |
| 2 | OCR Engine Mode | `String` | no | `LSTM only` | — | no | The OCR engine mode to use |

## How it works

Optical character recognition or optical character reader (OCR) is the mechanical or electronic
        conversion of images of typed, handwritten or printed text into machine-encoded text.


        Supported image formats: png, jpg, bmp, pbm.


        Requires Tesseract library. Enable with: --features tesseract

## Implementation

The implementation is in `src/operations/optical_character_recognition.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Optical Character Recognition"
```

For file or binary input use `rxchef run "Optical Character Recognition" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Optical Character Recognition" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/optical_character_recognition.rs

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
