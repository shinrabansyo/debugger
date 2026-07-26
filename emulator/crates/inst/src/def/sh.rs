use sb_emulator_inst_macros::S_style;
use sb_emulator_state::{InstType, State};

use crate::Inst;

#[S_style(0b011001)]
pub struct Sh;

impl Inst for Sh {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let rs2 = (state.regs.read(self.rs2)? & 0xFFFF) as u16;
        let addr = (rs1 + self.imm) as usize;
        state.dmem.write_half(addr, rs2)?;
        state.add_trace(
            InstType::MemWrite,
            Some(addr),
            Some((rs2 & 0xFFFF) as i32),
            None,
        );
        state.pc += 6;
        Ok(state)
    }
}
