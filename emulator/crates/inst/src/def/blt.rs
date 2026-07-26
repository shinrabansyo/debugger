use sb_emulator_inst_macros::B_style;
use sb_emulator_state::{InstType, State};

use crate::Inst;

#[B_style(0b100010)]
pub struct Blt;

impl Inst for Blt {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1_s = state.regs.read(self.rs1)? as i32;
        let rs2_s = state.regs.read(self.rs2)? as i32;
        let result = (state.pc + 6) as i32;
        if rs1_s < rs2_s {
            state.regs.write(self.rd, result)?;
            state.add_trace(InstType::RegWrite, None, Some(result), Some(self.rd));
            state.pc = ((state.pc as i32) + self.imm).try_into()?;
        } else {
            state.add_trace(InstType::RegWrite, None, None, None);
            state.pc += 6;
        }
        Ok(state)
    }
}
