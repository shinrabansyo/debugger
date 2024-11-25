use sb_emu_inst_macros::I_style;
use sb_emu_state::State;

use crate::Inst;

#[I_style(0b00100, 0b000)]
pub struct Lw;

impl Inst for Lw {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let addr = (rs1 + self.imm).try_into()?;
        let data = state.dmem.read_word(addr)?;
        state.regs.write(self.rd, data as i32)?;
        state.pc += 6;
        Ok(state)
    }
}
