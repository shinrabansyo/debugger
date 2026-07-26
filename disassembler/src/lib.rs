mod style;

use style::*;

pub fn disassemble(raw_inst: u64) -> String {
    match (raw_inst >> 32) & 0b111111 {
        0b000000 => i_common("addi", raw_inst),
        0b000001 => i_common("subi", raw_inst),
        0b000010 => i_common("andi", raw_inst),
        0b000011 => i_common("ori", raw_inst),
        0b000100 => i_common("xori", raw_inst),
        0b000101 => i_common("srli", raw_inst),
        0b000110 => i_common("srai", raw_inst),
        0b000111 => i_common("slli", raw_inst),
        0b001000 => r_common("add", raw_inst),
        0b001001 => r_common("sub", raw_inst),
        0b001010 => r_common("and", raw_inst),
        0b001011 => r_common("or", raw_inst),
        0b001100 => r_common("xor", raw_inst),
        0b001101 => r_common("srl", raw_inst),
        0b001110 => r_common("sra", raw_inst),
        0b001111 => r_common("sll", raw_inst),
        0b010000 => i_load("lw", raw_inst),
        0b010001 => i_load("lh", raw_inst),
        0b010010 => i_load("lb", raw_inst),
        0b010011 => i_load("lhu", raw_inst),
        0b010100 => i_load("lbu", raw_inst),
        0b010101 => i_load("ilb", raw_inst),
        0b010110 => i_load("in", raw_inst),
        0b011000 => s_common("sw", raw_inst),
        0b011001 => s_common("sh", raw_inst),
        0b011010 => s_common("sb", raw_inst),
        0b011101 => s_common("isb", raw_inst),
        0b011110 => s_common("out", raw_inst),
        0b100000 => b_common("beq", raw_inst),
        0b100001 => b_common("bne", raw_inst),
        0b100010 => b_common("blt", raw_inst),
        0b100011 => b_common("ble", raw_inst),
        0b100100 => i_common("jal", raw_inst),
        opcode => format!("unknown 0b{opcode:06b}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_isa_v2_fields() {
        let i = (-5_i32 as u32 as u64) | (0b000000_u64 << 32) | (30_u64 << 38) | (31_u64 << 43);
        assert_eq!(disassemble(i), "addi r31 = r30, -5");

        let r = (29_u64 << 27) | (0b001000_u64 << 32) | (30_u64 << 38) | (31_u64 << 43);
        assert_eq!(disassemble(r), "add  r31 = r30, r29");

        let b = 0x07ff_fffa_u64
            | (29_u64 << 27)
            | (0b100000_u64 << 32)
            | (30_u64 << 38)
            | (31_u64 << 43);
        assert_eq!(disassemble(b), "beq  r31, (r30, r29) -> -6");
    }

    #[test]
    fn reports_unknown_opcode() {
        assert_eq!(disassemble(0b11_1111_u64 << 32), "unknown 0b111111");
    }
}
