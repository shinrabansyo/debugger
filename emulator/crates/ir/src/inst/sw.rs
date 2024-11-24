use sb_emu_ir_macros::S_style;
use sb_emu_state::State;

use crate::inst::Inst;

#[S_style(0b00101, 0b000)]
pub struct Sw;

impl Inst for Sw {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let rs2 = state.regs.read(self.rs2)?;
        let addr = (rs1 + self.imm) as usize;
        state.dmem.write_word(addr, rs2)?;
        state.pc += 6;
        Ok(state)
    }
}
