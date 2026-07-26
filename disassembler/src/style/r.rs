pub fn r_common(name: &str, raw_inst: u64) -> String {
    let rd = (raw_inst >> 43) & 0b11111;
    let rs1 = (raw_inst >> 38) & 0b11111;
    let rs2 = (raw_inst >> 27) & 0b11111;

    format!("{:<4} r{} = r{}, r{}", name, rd, rs1, rs2)
}
