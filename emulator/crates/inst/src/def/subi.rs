use sb_emu_inst_macros::I_style;
use sb_emu_state::State;

use crate::Inst;

#[I_style(0b00010, 0b010)]
pub struct Subi;

impl Inst for Subi {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        state.regs.write(self.rd, rs1 - self.imm)?;
        state.pc += 6;
        Ok(state)
    }
}
