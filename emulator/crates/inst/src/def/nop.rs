use sb_emulator_inst_macros::R_style;
use sb_emulator_state::{InstType, State};

use crate::Inst;

#[R_style(0b00000, 0b000)]
pub struct Nop;

impl Inst for Nop {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        state.add_trace(InstType::RegWrite, None, None, None);
        state.pc += 6;
        Ok(state)
    }
}
