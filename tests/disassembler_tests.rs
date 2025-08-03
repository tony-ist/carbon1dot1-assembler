use carbon1dot1_assembler::disassembler::disassemble;

#[test]
fn test_disassemble_basic_instructions() {
    // Test basic instructions without operands
    let machine_code = vec![
        0b00000000, // nop
        0b11110000, // hlt (30 << 3)
        0b10101000, // ret (21 << 3)
        0b11000000, // psh (24 << 3)
        0b11001000, // pop (25 << 3)
    ];
    
    let disassembled = disassemble(&machine_code);
    
    // Check that the output contains the expected instructions
    assert!(disassembled[0].contains("nop"));
    assert!(disassembled[1].contains("hlt"));
    assert!(disassembled[2].contains("ret"));
    assert!(disassembled[3].contains("psh"));
    assert!(disassembled[4].contains("pop"));
}

#[test]
fn test_disassemble_register_operands() {
    let machine_code = vec![
        0b00001001, // inc r1
        0b00010010, // dec r2  
        0b00011011, // add r3
        0b00101000, // neg r0
    ];
    
    let disassembled = disassemble(&machine_code);
    
    // Check that the output contains the expected instructions with registers
    assert!(disassembled[0].contains("inc r1"));
    assert!(disassembled[1].contains("dec r2"));
    assert!(disassembled[2].contains("add r3"));
    assert!(disassembled[3].contains("neg r0"));
}

#[test]
fn test_disassemble_immediate_operands() {
    let machine_code = vec![
        0b01111001, // lim r1
        42,         // immediate value
        0b01100011, // bsl 3
        0b01101111, // bsr 7
    ];
    
    let disassembled = disassemble(&machine_code);
    
    assert!(disassembled[0].contains("lim r1"));
    assert!(disassembled[0].contains("0x2a") || disassembled[0].contains("42"));
    assert!(disassembled[1].contains("bsl"));
    assert!(disassembled[2].contains("bsr"));
}

#[test]
fn test_disassemble_address_operands() {
    let machine_code = vec![
        0b11010001, // pst $1 (26 << 3 | 1)
        0b11100010, // pld $2 (28 << 3 | 2)
        0b11011000, // psi $0 (27 << 3 | 0)
        123,        // immediate value for psi
    ];
    
    let disassembled = disassemble(&machine_code);

    assert!(disassembled[0].contains("pst") && disassembled[0].contains("$1"));
    assert!(disassembled[1].contains("pld") && disassembled[1].contains("$2"));
    assert!(disassembled[2].contains("psi") && disassembled[2].contains("$0"));
}

#[test]
fn test_disassemble_branch_instructions() {
    let machine_code = vec![
        // brc jmp .label
        0b10110000, 0x34, 0x56, // cond 0: JMP
        // brc even .label
        0b10110001, 0x34, 0x56, // cond 1: EVEN
        // brc eq .label
        0b10110010, 0x34, 0x56, // cond 2: EQ
        // brc neq .label
        0b10110011, 0x34, 0x56, // cond 3: NEQ
        // brc gt .label
        0b10110100, 0x34, 0x56, // cond 4: GT
        // brc lt .label
        0b10110101, 0x34, 0x56, // cond 5: LT
        // brc gteq .label
        0b10110110, 0x34, 0x56, // cond 6: GTEQ
        // brc lteq .label
        0b10110111, 0x34, 0x56, // cond 7: LTEQ
    ];

    let disassembled = disassemble(&machine_code);

    // There should be 8 branch instructions
    assert_eq!(disassembled.len(), 8);

    // Check that each branch instruction contains the correct mnemonic
    assert!(disassembled[0].to_lowercase().contains("brc") && disassembled[0].to_lowercase().contains("jmp"));
    assert!(disassembled[1].to_lowercase().contains("brc") && disassembled[1].to_lowercase().contains("even"));
    assert!(disassembled[2].to_lowercase().contains("brc") && disassembled[2].to_lowercase().contains("eq"));
    assert!(disassembled[3].to_lowercase().contains("brc") && disassembled[3].to_lowercase().contains("neq"));
    assert!(disassembled[4].to_lowercase().contains("brc") && disassembled[4].to_lowercase().contains("gt"));
    assert!(disassembled[5].to_lowercase().contains("brc") && disassembled[5].to_lowercase().contains("lt"));
    assert!(disassembled[6].to_lowercase().contains("brc") && disassembled[6].to_lowercase().contains("gteq"));
    assert!(disassembled[7].to_lowercase().contains("brc") && disassembled[7].to_lowercase().contains("lteq"));

    for line in &disassembled {
        assert!(line.contains("0x3456"), "Branch address missing in: {}", line);
    }
}
