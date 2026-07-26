use sb_emulator_inst_macros::S_style;
use sb_emulator_state::{InstType, State};

use crate::Inst;

#[S_style(0b011110)]
pub struct Out;

impl Inst for Out {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let rs2 = state.regs.read(self.rs2)?;
        let addr = (rs1 + self.imm).try_into()?;
        state.devices.write(addr, rs2 as u32)?;
        state.add_trace(InstType::DevWrite, Some(addr), Some(rs2), None);
        state.pc += 6;
        Ok(state)
    }
}
