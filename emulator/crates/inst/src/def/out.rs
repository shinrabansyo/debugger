use sb_emu_inst_macros::S_style;
use sb_emu_state::State;

use crate::Inst;

#[S_style(0b00110, 0b001)]
pub struct Out;

impl Inst for Out {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let rs2 = state.regs.read(self.rs2)?;
        let addr = (rs1 + self.imm) as usize;
        state.devices.write(addr, rs2)?;
        state.pc += 6;
        Ok(state)
    }
}
