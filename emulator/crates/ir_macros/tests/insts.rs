use sb_emu_ir_macros::{I_style, S_style, R_style, B_style};

#[test]
fn check_compile_i() {
    #[I_style]
    struct TestInst;

    const RAW_INST: u64 = 0x0
        | (0b10101    << 0)   // opcode
        | (0b111      << 5)   // opcode_sub
        | (0b00100    << 8)   // rd
        | (0b101      << 13)  // rs1
        | (0x12345678 << 18); // imm

    let inst = TestInst::from(RAW_INST);
    assert_eq!(inst.opcode, 0b10101);
    assert_eq!(inst.opcode_sub, 0b111);
    assert_eq!(inst.rd, 0b00100);
    assert_eq!(inst.rs1, 0b101);
    assert_eq!(inst.imm, 0x12345678);
}

#[test]
fn check_compile_s() {
    #[S_style]
    struct TestInst;

    const RAW_INST: u64 = 0x0
        | (0b10101    << 0)   // opcode
        | (0b111      << 5)   // opcode_sub
        | (0b00100    << 8)   // rs2
        | (0b101      << 13)  // rs1
        | (0x12345678 << 18); // imm

    let inst = TestInst::from(RAW_INST);
    assert_eq!(inst.opcode, 0b10101);
    assert_eq!(inst.opcode_sub, 0b111);
    assert_eq!(inst.rs2, 0b00100);
    assert_eq!(inst.rs1, 0b101);
    assert_eq!(inst.imm, 0x12345678);
}

#[test]
fn check_compile_r() {
    #[R_style]
    struct TestInst;

    const RAW_INST: u64 = 0x0
        | (0b10101    << 0)   // opcode
        | (0b111      << 5)   // opcode_sub
        | (0b00100    << 8)   // rd
        | (0b10100    << 13)  // rs1
        | (0b00101    << 18); // rs2

    let inst = TestInst::from(RAW_INST);
    assert_eq!(inst.opcode, 0b10101);
    assert_eq!(inst.opcode_sub, 0b111);
    assert_eq!(inst.rd, 0b00100);
    assert_eq!(inst.rs1, 0b10100);
    assert_eq!(inst.rs2, 0b00101);
}

#[test]
fn check_compile_b() {
    #[B_style]
    struct TestInst;

    const RAW_INST: u64 = 0x0
        | (0b10101    << 0)   // opcode
        | (0b111      << 5)   // opcode_sub
        | (0b00100    << 8)   // rd
        | (0b10100    << 13)  // rs1
        | (0b00101    << 18)  // rs2
        | (0xff345678 << 23); // imm

    let inst = TestInst::from(RAW_INST);
    assert_eq!(inst.opcode, 0b10101);
    assert_eq!(inst.opcode_sub, 0b111);
    assert_eq!(inst.rd, 0b00100);
    assert_eq!(inst.rs1, 0b10100);
    assert_eq!(inst.rs2, 0b00101);
    assert_eq!(inst.imm, 0x1345678);
}
