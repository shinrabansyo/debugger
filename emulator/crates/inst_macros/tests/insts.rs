use sb_emulator_inst_macros::{B_style, I_style, R_style, S_style};

#[test]
fn check_compile_i() {
    #[I_style(0b100101)]
    struct TestInst;

    const RAW_INST: u64 = 0x0
        | 0x12345678          // imm
        | (0b100101 << 32)    // opcode
        | (0b10101 << 38)     // rs1
        | (0b00100 << 43); // rd

    let inst = TestInst::from(RAW_INST);
    assert_eq!(inst.rd, 0b00100);
    assert_eq!(inst.rs1, 0b10101);
    assert_eq!(inst.imm, 0x12345678);
}

#[test]
fn check_compile_s() {
    #[S_style(0b100101)]
    struct TestInst;

    const RAW_INST: u64 = 0x0
        | 0x12345678          // imm
        | (0b100101 << 32)    // opcode
        | (0b10101 << 38)     // rs1
        | (0b00100 << 43); // rs2

    let inst = TestInst::from(RAW_INST);
    assert_eq!(inst.rs2, 0b00100);
    assert_eq!(inst.rs1, 0b10101);
    assert_eq!(inst.imm, 0x12345678);
}

#[test]
fn check_compile_r() {
    #[R_style(0b100101)]
    struct TestInst;

    const RAW_INST: u64 = 0x0
        | (0b00101 << 27)  // rs2
        | (0b100101 << 32) // opcode
        | (0b10100 << 38)  // rs1
        | (0b00100 << 43); // rd

    let inst = TestInst::from(RAW_INST);
    assert_eq!(inst.rd, 0b00100);
    assert_eq!(inst.rs1, 0b10100);
    assert_eq!(inst.rs2, 0b00101);
}

#[test]
fn check_compile_b() {
    #[B_style(0b100101)]
    struct TestInst;

    const RAW_INST: u64 = 0x0
        | 0x7345678          // imm 
        | (0b00101 << 27)    // rs2
        | (0b100101 << 32)   // opcode
        | (0b10100 << 38)    // rs1
        | (0b00100 << 43); // rd

    let inst = TestInst::from(RAW_INST);
    assert_eq!(inst.rd, 0b00100);
    assert_eq!(inst.rs1, 0b10100);
    assert_eq!(inst.rs2, 0b00101);
    assert_eq!(inst.imm, 0xff345678);
}
