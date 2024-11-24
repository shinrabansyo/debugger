use sb_emu_ir_macros::S_style;
use sb_emu_state::State;

use crate::inst::Inst;

#[S_style(0b00101, 0b001)]
pub struct Sh;

impl Inst for Sh {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let rs2 = (state.regs.read(self.rs2)? & 0xFFFF) as u16;
        let addr = (rs1 + self.imm) as usize;
        state.dmem.write_half(addr, rs2)?;
        state.pc += 6;
        Ok(state)
    }
}
