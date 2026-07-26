pub fn b_common(name: &str, raw_inst: u64) -> String {
    let rd = (raw_inst >> 43) & 0b11111;
    let rs1 = (raw_inst >> 38) & 0b11111;
    let rs2 = (raw_inst >> 27) & 0b11111;
    let imm = raw_inst & 0x7ffffff;
    let imm_s = sext_u27(imm);

    format!("{:<4} r{}, (r{}, r{}) -> {}", name, rd, rs1, rs2, imm_s)
}

fn sext_u27(value: u64) -> i32 {
    if (value >> 26) & 1 == 1 {
        let extended_value = (value as u32) | 0xF8000000;
        extended_value as i32
    } else {
        value as i32
    }
}
