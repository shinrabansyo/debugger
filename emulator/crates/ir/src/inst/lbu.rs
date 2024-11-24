use sb_emu_ir_macros::I_style;
use sb_emu_state::State;

use crate::inst::Inst;

#[I_style(0b00100, 0b100)]
pub struct Lbu;

impl Inst for Lbu {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let addr = (rs1 + self.imm) as usize;
        let data = state.dmem.read_byte(addr)? as u32;
        state.regs.write(self.rd, data)?;
        state.pc += 6;
        Ok(state)
    }
}
