use sb_emu_ir_macros::R_style;
use sb_emu_state::State;

use crate::inst::Inst;

#[R_style(0b00111, 0b100)]
pub struct Sra;

impl Inst for Sra {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)? as i32;
        let rs2 = state.regs.read(self.rs2)?;
        state.regs.write(self.rd, (rs1 >> rs2) as u32)?;
        state.pc += 6;
        Ok(state)
    }
}
