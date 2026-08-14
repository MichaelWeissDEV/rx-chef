# Generic Code Beautify

## Overview

Attempts to pretty print C-style languages such as C, C++, C#, Java, PHP, JavaScript etc.<br><br>This will not do a perfect job, and the resulting code may not work any more. This operation is designed purely to make obfuscated or minified code more easy to read and understand.<br><br>Things which will not work properly:<ul><li>For loop formatting</li><li>Do-While loop formatting</li><li>Switch/Case indentation</li><li>Certain bit shift operators</li></ul>

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

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

This operation has no arguments.

## How it works

Attempts to pretty print C-style languages such as C, C++, C#, Java, PHP, JavaScript etc.<br><br>This will not do a perfect job, and the resulting code may not work any more. This operation is designed purely to make obfuscated or minified code more easy to read and understand.<br><br>Things which will not work properly:<ul><li>For loop formatting</li><li>Do-While loop formatting</li><li>Switch/Case indentation</li><li>Certain bit shift operators</li></ul>

## Implementation

The implementation is in `src/operations/generic_code_beautify.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Generic Code Beautify"
```

For file or binary input use `rxchef run "Generic Code Beautify" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Generic Code Beautify" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/generic_code_beautify.rs

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
