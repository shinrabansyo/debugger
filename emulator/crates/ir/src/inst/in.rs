use sb_emu_ir_macros::I_style;
use sb_emu_state::State;

use crate::inst::Inst;

#[I_style(0b00110, 0b000)]
pub struct In;

impl Inst for In {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let addr = (rs1 + self.imm) as usize;
        let data = state.devices.read(addr)?;
        state.regs.write(self.rd, data)?;
        state.pc += 6;
        Ok(state)
    }
}
