use sb_emu_inst_macros::I_style;
use sb_emu_state::State;

use crate::Inst;

#[I_style(0b00011, 0b100)]
pub struct Jal;

impl Inst for Jal {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        state.regs.write(self.rd, (state.pc as i32) + 6)?;
        state.pc = (rs1 + self.imm).try_into()?;
        Ok(state)
    }
}
