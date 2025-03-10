use sb_emu_inst_macros::I_style;
use sb_emu_state::State;

use crate::Inst;

#[I_style(0b01000, 0b011)]
pub struct Srli;

impl Inst for Srli {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)? as u32;
        state.regs.write(self.rd, (rs1 >> self.imm) as i32)?;
        state.pc += 6;
        Ok(state)
    }
}
