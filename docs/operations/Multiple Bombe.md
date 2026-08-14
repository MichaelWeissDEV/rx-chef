# Multiple Bombe

## Overview

Emulation of the Bombe machine used to attack Enigma. This version carries out multiple Bombe runs to handle unknown rotor configurations.

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

Declared output type: `JSON`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Standard Enigmas | `String` | no | `German Service Enigma (First - 3 rotor)` | — | no | Preset rotor configurations |
| 2 | Main rotors | `String` | no | `<empty>` | — | no | Newline separated rotor wirings |
| 3 | 4th rotor | `String` | no | `<empty>` | — | no | Newline separated 4th rotor wirings |
| 4 | Reflectors | `String` | no | `<empty>` | — | no | Newline separated reflector pairs |
| 5 | Crib | `String` | no | `<empty>` | — | no | Known plaintext |
| 6 | Crib offset | `Integer` | no | `0` | — | no | Offset of the crib in the ciphertext |
| 7 | Use checking machine | `Boolean` | no | `true` | — | no | Whether to use the checking machine |

## How it works

Emulation of the Bombe machine used to attack Enigma. This version carries out multiple Bombe runs to handle unknown rotor configurations.

## Implementation

The implementation is in `src/operations/multiple_bombe.rs` and declares `String` input and `JSON` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Multiple Bombe"
```

For file or binary input use `rxchef run "Multiple Bombe" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Multiple Bombe" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/multiple_bombe.rs

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
