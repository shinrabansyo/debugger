use sb_emu_ir_macros::S_style;
use sb_emu_state::State;

use crate::inst::Inst;

#[S_style(0b00101, 0b010)]
pub struct Sb;

impl Inst for Sb {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let rs2 = (state.regs.read(self.rs2)? & 0xFF) as u8;
        let addr = (rs1 + self.imm) as usize;
        state.dmem.write_byte(addr, rs2)?;
        state.pc += 6;
        Ok(state)
    }
}
