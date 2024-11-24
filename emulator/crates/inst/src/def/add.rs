use sb_emu_inst_macros::R_style;
use sb_emu_state::State;

use crate::Inst;

#[R_style(0b00001, 0b001)]
pub struct Add;

impl Inst for Add {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let rs2 = state.regs.read(self.rs2)?;
        state.regs.write(self.rd, rs1 + rs2)?;
        state.pc += 6;
        Ok(state)
    }
}
