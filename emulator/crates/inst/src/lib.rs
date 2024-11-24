mod def;

use sb_emu_state::State;

pub use def::*;

pub trait Inst {
    fn exec(&self, state: State) -> anyhow::Result<State>;
}

pub fn parse(bytes: &[u8]) -> Box<dyn Inst> {
    let raw_inst =
        ((bytes[5] as u64) << 40) |
        ((bytes[4] as u64) << 32) |
        ((bytes[3] as u64) << 24) |
        ((bytes[2] as u64) << 16) |
        ((bytes[1] as u64) <<  8) |
        ((bytes[0] as u64) <<  0);

    match (raw_inst & 0b11111, (raw_inst >> 5) & 0b111) {
        (0b00000, 0b000) => Box::new(Nop::from(raw_inst)),
        (0b00001, 0b001) => Box::new(Add::from(raw_inst)),
        (0b00001, 0b010) => Box::new(Sub::from(raw_inst)),
        (0b00010, 0b001) => Box::new(Addi::from(raw_inst)),
        (0b00010, 0b010) => Box::new(Subi::from(raw_inst)),
        (0b00011, 0b000) => Box::new(Beq::from(raw_inst)),
        (0b00011, 0b001) => Box::new(Bne::from(raw_inst)),
        (0b00011, 0b010) => Box::new(Blt::from(raw_inst)),
        (0b00011, 0b011) => Box::new(Ble::from(raw_inst)),
        (0b00011, 0b100) => Box::new(Jal::from(raw_inst)),
        (0b00100, 0b000) => Box::new(Lw::from(raw_inst)),
        (0b00100, 0b001) => Box::new(Lh::from(raw_inst)),
        (0b00100, 0b010) => Box::new(Lb::from(raw_inst)),
        (0b00100, 0b011) => Box::new(Lhu::from(raw_inst)),
        (0b00100, 0b100) => Box::new(Lbu::from(raw_inst)),
        (0b00101, 0b000) => Box::new(Sw::from(raw_inst)),
        (0b00101, 0b001) => Box::new(Sh::from(raw_inst)),
        (0b00101, 0b010) => Box::new(Sb::from(raw_inst)),
        (0b00110, 0b000) => Box::new(In::from(raw_inst)),
        (0b00110, 0b001) => Box::new(Out::from(raw_inst)),
        (0b00111, 0b000) => Box::new(And::from(raw_inst)),
        (0b00111, 0b001) => Box::new(Or::from(raw_inst)),
        (0b00111, 0b010) => Box::new(Xor::from(raw_inst)),
        (0b00111, 0b011) => Box::new(Srl::from(raw_inst)),
        (0b00111, 0b100) => Box::new(Sra::from(raw_inst)),
        (0b00111, 0b101) => Box::new(Sll::from(raw_inst)),
        (0b01000, 0b000) => Box::new(Andi::from(raw_inst)),
        (0b01000, 0b001) => Box::new(Ori::from(raw_inst)),
        (0b01000, 0b010) => Box::new(Xori::from(raw_inst)),
        (0b01000, 0b011) => Box::new(Srli::from(raw_inst)),
        (0b01000, 0b100) => Box::new(Srai::from(raw_inst)),
        (0b01000, 0b101) => Box::new(Slli::from(raw_inst)),
        _ => unreachable!(),
    }
}
