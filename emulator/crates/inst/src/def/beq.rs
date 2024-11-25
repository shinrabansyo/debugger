use sb_emu_inst_macros::B_style;
use sb_emu_state::State;

use crate::Inst;

#[B_style(0b00011, 0b000)]
pub struct Beq;

impl Inst for Beq {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let rs2 = state.regs.read(self.rs2)?;
        if rs1 == rs2 {
            state.regs.write(self.rd, (state.pc + 6) as i32)?;
            state.pc = ((state.pc as i32) + self.imm).try_into()?;
        } else {
            state.pc += 6;
        }
        Ok(state)
    }
}
