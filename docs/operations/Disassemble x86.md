# Disassemble x86

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

## Overview

Disassembly is the process of translating machine language into assembly language.<br><br>This operation supports 64-bit, 32-bit and 16-bit code written for Intel or AMD x86 processors. It is particularly useful for reverse engineering shellcode.<br><br>Input should be in hexadecimal.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | FeatureDisabled |
| Features | disassembly |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `String`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Bit mode | `Enum` | no | `64` | 64, 32, 16 | no | The bit mode (64, 32, 16) |
| 2 | Compatibility | `String` | no | `Full x86 architecture` | — | no | The compatibility mode (not all supported by Capstone) |
| 3 | Code Segment (CS) | `Integer` | no | `0` | — | no | The code segment (not used by Capstone) |
| 4 | Offset (IP) | `Integer` | no | `0` | — | no | The instruction pointer offset |
| 5 | Show instruction hex | `Boolean` | no | `true` | — | no | Whether to show instruction hex |
| 6 | Show instruction position | `Boolean` | no | `true` | — | no | Whether to show instruction position |

## How it works

Disassembly is the process of translating machine language into assembly language.<br><br>This operation supports 64-bit, 32-bit and 16-bit code written for Intel or AMD x86 processors. It is particularly useful for reverse engineering shellcode.<br><br>Input should be in hexadecimal.

## Implementation

The implementation is in `src/operations/disassemble_x86.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Disassemble x86"
```

For file or binary input use `rxchef run "Disassemble x86" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Disassemble x86" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/disassemble_x86.rs

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
