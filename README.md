# Carbon1.1 Instruction Set Architecture (ISA)

## CPU Specifications

- **Clock Speed**: 8 Ticks (1.25 Hz) 5-Stage Pipeline
- **Architecture**: Pure Harvard (ACC Based)
- **ROM**: Supports up to 32 KiB (32,768 Bytes)  
- **RAM**: Supports up to 256B of external RAM
- **iCache**: 128 Bytes (single line 2t serial)
- **dCache**: 32 Bytes (4-line, serial 2t) in-direct addressing
- **Registers**: 7 general purpose registers (7B)
- **Operands**: 1op, 3bit
- **Opcodes**: 5 bit
- **PC**: Lower 7b(iCache), Upper 8b(ROM page address)
- **Hazards**: Taken care of by forwarder

## Instruction Format

Carbon1.1 uses variable-length instruction encoding with 1, 2, or 3 bytes:

### 1-Byte Instructions
Most instructions use a single byte with the format:
- **Bits 7-3**: 5-bit opcode
- **Bits 2-0**: 3-bit operand (register, condition, or immediate)

**Note**: BSL and BSR instructions encode the shift amount (0-7) directly in the 3-bit operand field.

### 2-Byte Instructions
Some instructions with 8-bit immediate values use two bytes:
- **Byte 1**: `[opcode(5-bit) + operand(3-bit)]`
- **Byte 2**: `[immediate_value(8-bit)]`

### 3-Byte Instructions  
Branch and call instructions use three bytes:
- **Byte 1**: `[opcode(5-bit) + operand(3-bit)]`
- **Byte 2**: `[page_address(8-bit)]`
- **Byte 3**: `[page_offset(8-bit)]`

## Instruction Encoding Summary

| Format | Instructions | Byte 1 | Byte 2 | Byte 3 |
|--------|-------------|---------|---------|---------|
| **1-byte** | Most instructions | `opcode + operand` | - | - |
| **2-byte** | LIM, PSI | `opcode + operand` | `immediate` | - |
| **3-byte** | BRC, ~~CAL~~, ~~PRD~~ | `opcode + operand` | `page_addr` | `page_offset` |

## Complete Instruction Set

### No Operation
| Mnemonic | Opcode | Binary | Bytes | Description |
|----------|---------|---------|--------|-------------|
| NOP | 0 | 00000 | 1 | No Operation |

### Arithmetic Instructions
| Mnemonic | Opcode | Binary | Bytes | Description | Flags Updated |
|----------|---------|---------|--------|-------------|---------------|
| INC | 1 | 00001 | 1 | Increment register | Yes |
| DEC | 2 | 00010 | 1 | Decrement register | Yes |
| ADD | 3 | 00011 | 1 | Add register to accumulator (acc += reg[x]) | Yes |
| ADR | 4 | 00100 | 1 | Add accumulator to register (reg[x] += acc) | Yes |
| NEG | 5 | 00101 | 1 | Bitwise NOT register (reg[x] = ~reg[x]) | Yes |
| SUB | 6 | 00110 | 1 | Subtract register from accumulator (acc -= reg[x]) | Yes |
| BSB | 7 | 00111 | 1 | Backwards subtraction (acc = reg[x] - acc) | Yes |
| CMP | 8 | 01000 | 1 | Compare accumulator with register (FLAGS = acc - reg[x]) | Yes |

### Logic Instructions  
| Mnemonic | Opcode | Binary | Bytes | Description | Flags Updated |
|----------|---------|---------|--------|-------------|---------------|
| BOR | 9 | 01001 | 1 | Bitwise OR with register (acc = acc OR reg[x]) | Yes |
| AND | 10 | 01010 | 1 | Bitwise AND with register (acc = acc AND reg[x]) | Yes |
| XOR | 11 | 01011 | 1 | Bitwise XOR with register (acc = acc XOR reg[x]) | Yes |
| BSL | 12 | 01100 | 1 | Barrel Shift Left accumulator by 0-7 bits | Yes |
| BSR | 13 | 01101 | 1 | Barrel Shift Right accumulator by 0-7 bits | Yes |

### Memory & Register Instructions
| Mnemonic | Opcode | Binary | Bytes | Description | Flags Updated |
|----------|---------|---------|--------|-------------|---------------|
| LIM | 15 | 01111 | 2 | Load Immediate (reg[x] = IMM; acc = IMM if x=0) | No |
| RST | 16 | 10000 | 1 | Register Store (reg[x] = acc) | No |
| RLD | 17 | 10001 | 1 | Register Load (acc = reg[x]) | No |
| MST | 18 | 10010 | 1 | Memory Store (mem[reg[x]] = acc) | No |
| MLD | 19 | 10011 | 1 | Memory Load (acc = mem[reg[x]]) | No |

### Control Flow Instructions
| Mnemonic | Opcode | Binary | Bytes | Description | Flags Updated |
|----------|---------|---------|--------|-------------|---------------|
| CAL | 20 | 10100 | 3 | **[NOT IMPLEMENTED]** Call function | No |
| RET | 21 | 10101 | 1 | **[NOT IMPLEMENTED]** Return from function | No |
| BRC | 22 | 10110 | 3 | Branch on condition to page:offset | No |
| JID | 23 | 10111 | 1 | **[NOT IMPLEMENTED]** Jump Indirect | No |

### Stack Instructions
| Mnemonic | Opcode | Binary | Bytes | Description | Flags Updated |
|----------|---------|---------|--------|-------------|---------------|
| PSH | 24 | 11000 | 1 | Push accumulator to stack | No |
| POP | 25 | 11001 | 1 | Pop from stack to accumulator | No |

### I/O Port Instructions
| Mnemonic | Opcode | Binary | Bytes | Description | Flags Updated |
|----------|---------|---------|--------|-------------|---------------|
| PST | 26 | 11010 | 1 | Store accumulator to port (PORTS[x] = acc) | No |
| PSI | 27 | 11011 | 2 | Store immediate to port (PORTS[x] = IMM) | No |
| PLD | 28 | 11100 | 1 | Load from port to accumulator (acc = PORTS[x]) | Yes |
| PRD | 29 | 11101 | 3 | **[NOT IMPLEMENTED]** Predicate: disable next instruction if condition false | No |

### System Instructions
| Mnemonic | Opcode | Binary | Bytes | Description | Flags Updated |
|----------|---------|---------|--------|-------------|---------------|
| HLT | 30 | 11110 | 1 | Halt processor | No |
| FLS | 31 | 11111 | 1 | Flush | No |

## Branch Conditions

Branch and predicate instructions use 3-bit condition codes:

| Condition | Code | Binary | Description |
|-----------|------|---------|-------------|
| JMP | 0 | 000 | Jump (Unconditional) |
| EVEN | 1 | 001 | Even (LSB = 0) |
| EQ | 2 | 010 | Equal (A = B) |
| NEQ | 3 | 011 | Not Equal (A ≠ B) |
| GT | 4 | 100 | Greater Than (A > B) |
| LT | 5 | 101 | Less Than (A < B) |
| GTEQ | 6 | 110 | Greater Than or Equal (A ≥ B) (COUT) |
| LTEQ | 7 | 111 | Less Than or Equal (A ≤ B) |

## Register Encoding

- **R0**: Accumulator address (special handling - LIM R0 writes to accumulator)
- **R1-R7**: General purpose registers (3-bit encoding: 001-111)

## Flag-Updating Instructions

The following instructions update processor flags:
- All arithmetic instructions (INC, DEC, ADD, ADR, NEG, SUB, BSB)
- All logic instructions (BOR, AND, XOR, BSL, BSR)  
- Comparison instruction (CMP)
- Port load instruction (PLD)

## Special Notes

- **Accumulator (ACC)**: The primary working register
- **R0 Special Behavior**: Reading R0 returns 0 (not accumulator value), writing to R0 does nothing except for LIM R0 which writes to accumulator
- **Harvard Architecture**: Separate instruction and data memory spaces
- **Pipeline**: 5-stage pipeline (Fetch, Decode, Read, Execute, Writeback)
- **Opcode 14** is not used
- **⚠️ NOT IMPLEMENTED**: Instructions CAL, RET, JID, and PRD are defined in the ISA but not implemented in the actual CPU hardware. Do not use these instructions in programs intended to run on the physical processor.

## Multi-Byte Instruction Details

### BRC (Branch) Instruction Format
The BRC instruction is always 3 bytes:
- **Byte 1**: Instruction opcode (10110) + 3-bit condition code
- **Byte 2**: Page address (8-bit)  
- **Byte 3**: Address within the page (8-bit)

### 2-Byte Instructions
- **LIM**: `[opcode + register] [immediate_value]`
- **PSI**: `[opcode + port_address] [immediate_value]`

## Instruction Encoding Examples

### 1-Byte Instructions
- `NOP` → `0x00` (opcode 0 << 3 + 0)
- `INC R1` → `0x09` (opcode 1 << 3 + register 1)
- `BSL 3` → `0x63` (opcode 12 << 3 + shift_amount 3)
- `BSR 5` → `0x6D` (opcode 13 << 3 + shift_amount 5)
- `PSH` → `0xC0` (opcode 24 << 3 + 0)

### 2-Byte Instructions
- `LIM R1, 0x42` → `0x79 0x42` (opcode 15 << 3 + register 1, immediate)
- `PSI $3, 0xFF` → `0xDB 0xFF` (opcode 27 << 3 + port 3, immediate)

### 3-Byte Instructions
- `BRC EQ, 0x1234` → `0xB2 0x12 0x34` (opcode 22 << 3 + condition 2, page, offset)

## Address Space

- **16-bit addressing**: Used for branch instructions with page/offset format
- **Memory Layout**: Harvard architecture with separate instruction and data spaces