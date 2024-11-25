pub fn b_common(name: &str, raw_inst: u64) -> String {
    let rd = (raw_inst >> 8) & 0b11111;
    let rs1 = (raw_inst >> 13) & 0b11111;
    let rs2 = (raw_inst >> 18) & 0b11111;
    let imm = (raw_inst >> 23) & 0x1ffffff;
    let imm_s = sext_u25(imm);

    format!("{} r{}, (r{}, r{}) -> {}", name, rd, rs1, rs2, imm_s)
}

fn sext_u25(value: u64) -> i32 {
    if (value >> 24) & 1 == 1{
        let extended_value = (value as u32) | 0xFF000000;
        extended_value as i32
    } else {
        value as i32
    }
}
