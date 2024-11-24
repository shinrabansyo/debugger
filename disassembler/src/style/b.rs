pub fn b_common(name: &str, raw_inst: u64) -> String {
    let rd = (raw_inst >> 8) & 0b11111;
    let rs1 = (raw_inst >> 13) & 0b11111;
    let rs2 = (raw_inst >> 18) & 0b11111;
    let imm = (raw_inst >> 23) & 0x1ffffff;

    format!("{} r{}, (r{}, r{}) -> {}", name, rd, rs1, rs2, imm)
}
