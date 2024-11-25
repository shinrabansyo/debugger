use sb_emu_inst_macros::I_style;
use sb_emu_state::State;

use crate::Inst;

#[I_style(0b00100, 0b001)]
pub struct Lh;

impl Inst for Lh {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let addr = (rs1 + self.imm).try_into()?;
        let data = state.dmem.read_half(addr)?;
        state.regs.write(self.rd, sext(data))?;
        state.pc += 6;
        Ok(state)
    }
}

fn sext(data: u16) -> i32 {
    if data & 0x8000 != 0 {
        (data as u32 | 0xFFFF_0000) as i32
    } else {
        (data as u32) as i32
    }
}
