# Disassemble x86

Disassembly is the process of translating machine language into assembly language.<br><br>This operation supports 64-bit, 32-bit and 16-bit code written for Intel or AMD x86 processors. It is particularly useful for reverse engineering shellcode.<br><br>Input should be in hexadecimal.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Disassemble x86"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Bit mode | `64` | The bit mode (64, 32, 16) |
| 2 | Compatibility | `Full x86 architecture` | The compatibility mode (not all supported by Capstone) |
| 3 | Code Segment (CS) | `0` | The code segment (not used by Capstone) |
| 4 | Offset (IP) | `0` | The instruction pointer offset |
| 5 | Show instruction hex | `true` | Whether to show instruction hex |
| 6 | Show instruction position | `true` | Whether to show instruction position |

