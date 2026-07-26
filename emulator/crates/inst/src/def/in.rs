use sb_emulator_inst_macros::I_style;
use sb_emulator_state::{InstType, State};

use crate::Inst;

#[I_style(0b010110)]
pub struct In;

impl Inst for In {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let addr = (rs1 + self.imm).try_into()?;
        let data = state.devices.read(addr)?;
        state.regs.write(self.rd, data as i32)?;
        state.add_trace(
            InstType::DevRead,
            Some(addr),
            Some(data as i32),
            Some(self.rd),
        );
        state.pc += 6;
        Ok(state)
    }
}
