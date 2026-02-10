use sb_emulator_inst_macros::I_style;
use sb_emulator_state::{InstType, State};

use crate::Inst;

#[I_style(0b00010, 0b001)]
pub struct Addi;

impl Inst for Addi {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let result = rs1 + self.imm;
        state.regs.write(self.rd, result)?;
        state.add_trace(InstType::RegWrite, None, Some(result), Some(self.rd));
        state.pc += 6;
        Ok(state)
    }
}
