pub fn i_common(name: &str, raw_inst: u64) -> String {
    let rd = (raw_inst >> 43) & 0b11111;
    let rs1 = (raw_inst >> 38) & 0b11111;
    let imm = (raw_inst & 0xffffffff) as i32;

    format!("{:<4} r{} = r{}, {}", name, rd, rs1, imm)
}

pub fn i_load(name: &str, raw_inst: u64) -> String {
    let rd = (raw_inst >> 43) & 0b11111;
    let rs1 = (raw_inst >> 38) & 0b11111;
    let imm = (raw_inst & 0xffffffff) as i32;

    format!("{:<4} r{} = r{}[{}]", name, rd, rs1, imm)
}
