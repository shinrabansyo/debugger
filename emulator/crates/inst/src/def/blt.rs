use sb_emu_inst_macros::B_style;
use sb_emu_state::State;

use crate::Inst;

#[B_style(0b00011, 0b010)]
pub struct Blt;

impl Inst for Blt {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1_s = state.regs.read(self.rs1)? as i32;
        let rs2_s = state.regs.read(self.rs2)? as i32;
        if rs1_s < rs2_s {
            state.regs.write(self.rd, (state.pc as i32) + 6)?;
            state.pc = ((state.pc as i32) + self.imm).try_into()?;
        } else {
            state.pc += 6;
        }
        Ok(state)
    }
}
