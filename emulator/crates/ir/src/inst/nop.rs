use sb_emu_ir_macros::R_style;
use sb_emu_state::State;

use crate::inst::Inst;

#[R_style(0b00000, 0b000)]
pub struct Nop;

impl Inst for Nop {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        state.pc += 6;
        Ok(state)
    }
}
