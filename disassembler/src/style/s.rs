pub fn s_common(name: &str, raw_inst: u64) -> String {
    let rs2 = (raw_inst >> 8) & 0b11111;
    let rs1 = (raw_inst >> 13) & 0b111;
    let imm = ((raw_inst >> 16) & 0xffffffff) as i32;

    format!("{:<4} r{}[{}] = r{}", name, rs1, imm, rs2)
}
