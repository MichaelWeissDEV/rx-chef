# Disassemble ARM

Disassembles ARM machine code into assembly language.<br><br>Supports ARM (32-bit), Thumb, and ARM64 (AArch64) architectures using the Capstone disassembly framework.<br><br>Input should be in hexadecimal.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Disassemble ARM"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Architecture | `ARM (32-bit)` | The ARM architecture to use. |
| 2 | Mode | `ARM` | The instruction set mode. |
| 3 | Endianness | `Little Endian` | The byte order. |
| 4 | Starting address (hex) | `0` | The address to start disassembling from. |
| 5 | Show instruction hex | `true` | Whether to show the hex bytes of each instruction. |
| 6 | Show instruction position | `true` | Whether to show the address of each instruction. |

