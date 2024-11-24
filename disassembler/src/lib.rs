mod style;

use style::*;

pub fn disassemble(raw_inst: u64) -> String {
    match (raw_inst & 0b11111, (raw_inst >> 5) & 0b111) {
        (0b00000, 0b000) => r_common("nop", raw_inst),
        (0b00001, 0b001) => r_common("add", raw_inst),
        (0b00001, 0b010) => r_common("sub", raw_inst),
        (0b00010, 0b001) => i_common("addi", raw_inst),
        (0b00010, 0b010) => i_common("subi", raw_inst),
        (0b00011, 0b000) => b_common("beq", raw_inst),
        (0b00011, 0b001) => b_common("bne", raw_inst),
        (0b00011, 0b010) => b_common("blt", raw_inst),
        (0b00011, 0b011) => b_common("ble", raw_inst),
        (0b00011, 0b100) => i_common("jal", raw_inst),
        (0b00100, 0b000) => i_common("lw", raw_inst),
        (0b00100, 0b001) => i_common("lh", raw_inst),
        (0b00100, 0b010) => i_common("lb", raw_inst),
        (0b00100, 0b011) => i_common("lhu", raw_inst),
        (0b00100, 0b100) => i_common("lbu", raw_inst),
        (0b00101, 0b000) => s_common("sw", raw_inst),
        (0b00101, 0b001) => s_common("sh", raw_inst),
        (0b00101, 0b010) => s_common("sb", raw_inst),
        (0b00110, 0b000) => i_common("in", raw_inst),
        (0b00110, 0b001) => s_common("out", raw_inst),
        (0b00111, 0b000) => r_common("and", raw_inst),
        (0b00111, 0b001) => r_common("or", raw_inst),
        (0b00111, 0b010) => r_common("xor", raw_inst),
        (0b00111, 0b011) => r_common("srl", raw_inst),
        (0b00111, 0b100) => r_common("sra", raw_inst),
        (0b00111, 0b101) => r_common("sll", raw_inst),
        (0b01000, 0b000) => i_common("andi", raw_inst),
        (0b01000, 0b001) => i_common("ori", raw_inst),
        (0b01000, 0b010) => i_common("xori", raw_inst),
        (0b01000, 0b011) => i_common("srli", raw_inst),
        (0b01000, 0b100) => i_common("srai", raw_inst),
        (0b01000, 0b101) => i_common("slli", raw_inst),
        _ => unimplemented!(),
    }
}
