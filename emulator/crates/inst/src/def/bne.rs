use sb_emulator_inst_macros::B_style;
use sb_emulator_state::{InstType, State};

use crate::Inst;

#[B_style(0b00011, 0b001)]
pub struct Bne;

impl Inst for Bne {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let rs2 = state.regs.read(self.rs2)?;
        let result = (state.pc + 6) as i32;
        if rs1 != rs2 {
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
