mod def;

use sb_emu_state::State;

pub use def::*;

pub trait Inst {
    fn exec(&self, state: State) -> anyhow::Result<State>;
}

pub fn parse(raw_inst: u64) -> anyhow::Result<Box<dyn Inst>> {
    match (raw_inst & 0b11111, (raw_inst >> 5) & 0b111) {
        (0b00000, 0b000) => Ok(Box::new(Nop::from(raw_inst))),
        (0b00001, 0b001) => Ok(Box::new(Add::from(raw_inst))),
        (0b00001, 0b010) => Ok(Box::new(Sub::from(raw_inst))),
        (0b00010, 0b001) => Ok(Box::new(Addi::from(raw_inst))),
        (0b00010, 0b010) => Ok(Box::new(Subi::from(raw_inst))),
        (0b00011, 0b000) => Ok(Box::new(Beq::from(raw_inst))),
        (0b00011, 0b001) => Ok(Box::new(Bne::from(raw_inst))),
        (0b00011, 0b010) => Ok(Box::new(Blt::from(raw_inst))),
        (0b00011, 0b011) => Ok(Box::new(Ble::from(raw_inst))),
        (0b00011, 0b100) => Ok(Box::new(Jal::from(raw_inst))),
        (0b00100, 0b000) => Ok(Box::new(Lw::from(raw_inst))),
        (0b00100, 0b001) => Ok(Box::new(Lh::from(raw_inst))),
        (0b00100, 0b010) => Ok(Box::new(Lb::from(raw_inst))),
        (0b00100, 0b011) => Ok(Box::new(Lhu::from(raw_inst))),
        (0b00100, 0b100) => Ok(Box::new(Lbu::from(raw_inst))),
        (0b00101, 0b000) => Ok(Box::new(Sw::from(raw_inst))),
        (0b00101, 0b001) => Ok(Box::new(Sh::from(raw_inst))),
        (0b00101, 0b010) => Ok(Box::new(Sb::from(raw_inst))),
        (0b00110, 0b000) => Ok(Box::new(In::from(raw_inst))),
        (0b00110, 0b001) => Ok(Box::new(Out::from(raw_inst))),
        (0b00111, 0b000) => Ok(Box::new(And::from(raw_inst))),
        (0b00111, 0b001) => Ok(Box::new(Or::from(raw_inst))),
        (0b00111, 0b010) => Ok(Box::new(Xor::from(raw_inst))),
        (0b00111, 0b011) => Ok(Box::new(Srl::from(raw_inst))),
        (0b00111, 0b100) => Ok(Box::new(Sra::from(raw_inst))),
        (0b00111, 0b101) => Ok(Box::new(Sll::from(raw_inst))),
        (0b01000, 0b000) => Ok(Box::new(Andi::from(raw_inst))),
        (0b01000, 0b001) => Ok(Box::new(Ori::from(raw_inst))),
        (0b01000, 0b010) => Ok(Box::new(Xori::from(raw_inst))),
        (0b01000, 0b011) => Ok(Box::new(Srli::from(raw_inst))),
        (0b01000, 0b100) => Ok(Box::new(Srai::from(raw_inst))),
        (0b01000, 0b101) => Ok(Box::new(Slli::from(raw_inst))),
        _ => Err(anyhow::anyhow!("Unknown instruction")),
    }
}
