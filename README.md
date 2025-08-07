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

Instructions are encoded as 8-bit values with the following format:
- **Bits 7-3**: 5-bit opcode
- **Bits 2-0**: 3-bit operand (register or condition)

## Complete Instruction Set

### No Operation
| Mnemonic | Opcode | Binary | Description |
|----------|---------|---------|-------------|
| NOP | 0 | 00000 | No Operation |

### Arithmetic Instructions
| Mnemonic | Opcode | Binary | Description | Flags Updated |
|----------|---------|---------|-------------|---------------|
| INC | 1 | 00001 | Increment register | Yes |
| DEC | 2 | 00010 | Decrement register | Yes |
| ADD | 3 | 00011 | Add register to accumulator | Yes |
| ADR | 4 | 00100 | Add to Registers | Yes |
| NEG | 5 | 00101 | Negate register | Yes |
| SUB | 6 | 00110 | Subtract register from accumulator | Yes |
| BSB | 7 | 00111 | Backwards Subtraction | Yes |
| CMP | 8 | 01000 | Compare accumulator with register | Yes |

### Logic Instructions  
| Mnemonic | Opcode | Binary | Description | Flags Updated |
|----------|---------|---------|-------------|---------------|
| BOR | 9 | 01001 | Bitwise OR with register | Yes |
| AND | 10 | 01010 | Bitwise AND with register | Yes |
| XOR | 11 | 01011 | Bitwise XOR with register | Yes |
| BSL | 12 | 01100 | Barrel Shift Left by immediate | Yes |
| BSR | 13 | 01101 | Barrel Shift Right by immediate | Yes |

### Memory & Register Instructions
| Mnemonic | Opcode | Binary | Description | Flags Updated |
|----------|---------|---------|-------------|---------------|
| LIM | 15 | 01111 | Load Immediate to register | No |
| RST | 16 | 10000 | Register Store (acc → register) | No |
| RLD | 17 | 10001 | Register Load (register → acc) | No |
| MST | 18 | 10010 | Memory Store | No |
| MLD | 19 | 10011 | Memory Load | No |

### Control Flow Instructions
| Mnemonic | Opcode | Binary | Description | Flags Updated |
|----------|---------|---------|-------------|---------------|
| CAL | 20 | 10100 | Call function | No |
| RET | 21 | 10101 | Return from function | No |
| BRC | 22 | 10110 | Branch on condition (3-byte instruction) | No |
| JID | 23 | 10111 | Jump Indirect | No |

### Stack Instructions
| Mnemonic | Opcode | Binary | Description | Flags Updated |
|----------|---------|---------|-------------|---------------|
| PSH | 24 | 11000 | Push accumulator to stack | No |
| POP | 25 | 11001 | Pop from stack to accumulator | No |

### I/O Port Instructions
| Mnemonic | Opcode | Binary | Description | Flags Updated |
|----------|---------|---------|-------------|---------------|
| PST | 26 | 11010 | Store accumulator to port | No |
| PSI | 27 | 11011 | Store immediate to port | No |
| PLD | 28 | 11100 | Load from port to accumulator | Yes |
| PRD | 29 | 11101 | Predicate (conditional execution) | No |

### System Instructions
| Mnemonic | Opcode | Binary | Description | Flags Updated |
|----------|---------|---------|-------------|---------------|
| HLT | 30 | 11110 | Halt processor | No |
| FLS | 31 | 11111 | Flush | No |

## Branch Conditions

Branch and predicate instructions use 3-bit condition codes:

| Condition | Code | Binary | Description |
|-----------|------|---------|-------------|
| JMP | 0 | 000 | Jump (Unconditional) |
| EVEN | 1 | 001 | Even (Odd=lsb[0]) |
| EQ | 2 | 010 | Equal (A = B) |
| NEQ | 3 | 011 | Not Equal (A ≠ B) |
| GT | 4 | 100 | Greater Than (A > B) |
| LT | 5 | 101 | Less Than (A < B) |
| GTEQ | 6 | 110 | Greater Than or Equal (A ≥ B) |
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

## BRC Instruction Format

The BRC (Branch) instruction is always 3 bytes:
- **Byte 1**: Instruction opcode (10110) + 3-bit condition code
- **Byte 2**: Page address (8-bit)  
- **Byte 3**: Address within the page (7-bit)

## Address Space

- **16-bit addressing**: Used for branch instructions with page/offset format
- **Memory Layout**: Harvard architecture with separate instruction and data spaces