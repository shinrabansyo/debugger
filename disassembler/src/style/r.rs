pub fn r_common(name: &str, raw_inst: u64) -> String {
    let rd = (raw_inst >> 8) & 0b11111;
    let rs1 = (raw_inst >> 13) & 0b11111;
    let rs2 = (raw_inst >> 18) & 0b11111;

    format!("{} r{} = r{}, r{}", name, rd, rs1, rs2)
}

pub fn r_nop(_: &str, _: u64) -> String {
    format!("nop")
}
