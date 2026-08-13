# Disassemble ARM

## Overview

Disassembles ARM machine code into assembly language.<br><br>Supports ARM (32-bit), Thumb, and ARM64 (AArch64) architectures using the Capstone disassembly framework.<br><br>Input should be in hexadecimal.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | available |
| Features | none |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `String`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Architecture | `String` | no | `ARM (32-bit)` | — | no | The ARM architecture to use. |
| 2 | Mode | `String` | no | `ARM` | — | no | The instruction set mode. |
| 3 | Endianness | `String` | no | `Little Endian` | — | no | The byte order. |
| 4 | Starting address (hex) | `Integer` | no | `0` | — | no | The address to start disassembling from. |
| 5 | Show instruction hex | `Boolean` | no | `true` | — | no | Whether to show the hex bytes of each instruction. |
| 6 | Show instruction position | `Boolean` | no | `true` | — | no | Whether to show the address of each instruction. |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/disassemble_arm.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Disassemble ARM"
```

For file or binary input use `rxchef run "Disassemble ARM" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Disassemble ARM" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

The mapped Rust test and available KAT/differential/property/fuzz evidence are recorded in the [operation quality matrix](../reference/operation-matrix.md).

## Performance

See [benchmark results](../performance/results.md). Operations outside the representative catalog are explicitly marked with a skip rationale in the machine-readable quality inventory. Measurements are hardware-dependent reference values, not guarantees.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
