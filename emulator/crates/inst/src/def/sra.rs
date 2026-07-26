use sb_emulator_inst_macros::R_style;
use sb_emulator_state::{InstType, State};

use crate::Inst;

#[R_style(0b001110)]
pub struct Sra;

impl Inst for Sra {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let rs2 = state.regs.read(self.rs2)?;
        let result = rs1 >> rs2;
        state.regs.write(self.rd, result)?;
        state.add_trace(InstType::RegWrite, None, Some(result), Some(self.rd));
        state.pc += 6;
        Ok(state)
    }
}
