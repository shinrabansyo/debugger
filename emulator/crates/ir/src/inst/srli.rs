use sb_emu_ir_macros::I_style;
use sb_emu_state::State;

use crate::inst::Inst;

#[I_style(0b01000, 0b011)]
pub struct Srli;

impl Inst for Srli {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        state.regs.write(self.rd, rs1 >> self.imm)?;
        state.pc += 6;
        Ok(state)
    }
}
