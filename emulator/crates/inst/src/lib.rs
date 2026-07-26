mod def;

use sb_emulator_state::State;

pub use def::*;

pub trait Inst {
    fn exec(&self, state: State) -> anyhow::Result<State>;
}

pub fn parse(raw_inst: u64) -> anyhow::Result<Box<dyn Inst>> {
    match (raw_inst >> 32) & 0b111111 {
        0b000000 => Ok(Box::new(Addi::from(raw_inst))),
        0b000001 => Ok(Box::new(Subi::from(raw_inst))),
        0b000010 => Ok(Box::new(Andi::from(raw_inst))),
        0b000011 => Ok(Box::new(Ori::from(raw_inst))),
        0b000100 => Ok(Box::new(Xori::from(raw_inst))),
        0b000101 => Ok(Box::new(Srli::from(raw_inst))),
        0b000110 => Ok(Box::new(Srai::from(raw_inst))),
        0b000111 => Ok(Box::new(Slli::from(raw_inst))),
        0b001000 => Ok(Box::new(Add::from(raw_inst))),
        0b001001 => Ok(Box::new(Sub::from(raw_inst))),
        0b001010 => Ok(Box::new(And::from(raw_inst))),
        0b001011 => Ok(Box::new(Or::from(raw_inst))),
        0b001100 => Ok(Box::new(Xor::from(raw_inst))),
        0b001101 => Ok(Box::new(Srl::from(raw_inst))),
        0b001110 => Ok(Box::new(Sra::from(raw_inst))),
        0b001111 => Ok(Box::new(Sll::from(raw_inst))),
        0b010000 => Ok(Box::new(Lw::from(raw_inst))),
        0b010001 => Ok(Box::new(Lh::from(raw_inst))),
        0b010010 => Ok(Box::new(Lb::from(raw_inst))),
        0b010011 => Ok(Box::new(Lhu::from(raw_inst))),
        0b010100 => Ok(Box::new(Lbu::from(raw_inst))),
        0b010101 => Ok(Box::new(Ilb::from(raw_inst))),
        0b010110 => Ok(Box::new(In::from(raw_inst))),
        0b011000 => Ok(Box::new(Sw::from(raw_inst))),
        0b011001 => Ok(Box::new(Sh::from(raw_inst))),
        0b011010 => Ok(Box::new(Sb::from(raw_inst))),
        0b011101 => Ok(Box::new(Isb::from(raw_inst))),
        0b011110 => Ok(Box::new(Out::from(raw_inst))),
        0b100000 => Ok(Box::new(Beq::from(raw_inst))),
        0b100001 => Ok(Box::new(Bne::from(raw_inst))),
        0b100010 => Ok(Box::new(Blt::from(raw_inst))),
        0b100011 => Ok(Box::new(Ble::from(raw_inst))),
        0b100100 => Ok(Box::new(Jal::from(raw_inst))),
        opcode => Err(anyhow::anyhow!("Unknown instruction opcode: {opcode:06b}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i_inst(opcode: u64, rd: u64, rs1: u64, imm: i32) -> u64 {
        (imm as u32 as u64) | (opcode << 32) | (rs1 << 38) | (rd << 43)
    }

    fn b_inst(opcode: u64, rd: u64, rs1: u64, rs2: u64, imm: i32) -> u64 {
        ((imm as u32 as u64) & 0x07ff_ffff)
            | (rs2 << 27)
            | (opcode << 32)
            | (rs1 << 38)
            | (rd << 43)
    }

    #[test]
    fn executes_i_format_with_five_bit_registers() {
        let mut state = State::new(0, &[], &[]);
        state.regs.write(30, 42).unwrap();

        let state = parse(i_inst(0b000000, 31, 30, -5))
            .unwrap()
            .exec(state)
            .unwrap();

        assert_eq!(state.regs.read(31).unwrap(), 37);
        assert_eq!(state.pc, 6);
    }

    #[test]
    fn executes_negative_branch_offset() {
        let mut state = State::new(12, &[], &[]);
        state.regs.write(30, 7).unwrap();
        state.regs.write(29, 7).unwrap();

        let state = parse(b_inst(0b100000, 31, 30, 29, -6))
            .unwrap()
            .exec(state)
            .unwrap();

        assert_eq!(state.regs.read(31).unwrap(), 18);
        assert_eq!(state.pc, 6);
    }

    #[test]
    fn ilb_reads_signed_byte_from_instruction_memory() {
        let mut state = State::new(0, &[], &[]);
        state.regs.write(30, 96).unwrap();
        state.imem.write_byte(100, 0xfe).unwrap();

        let state = parse(i_inst(0b010101, 31, 30, 4))
            .unwrap()
            .exec(state)
            .unwrap();

        assert_eq!(state.regs.read(31).unwrap(), -2);
        assert_eq!(state.pc, 6);
    }

    #[test]
    fn rejects_unassigned_opcode() {
        assert!(parse(0b11_1111_u64 << 32).is_err());
    }
}
