use sb_emulator_inst_macros::I_style;
use sb_emulator_state::{InstType, State};

use crate::Inst;

#[I_style(0b010101)]
pub struct Ilb;

impl Inst for Ilb {
    fn exec(&self, mut state: State) -> anyhow::Result<State> {
        let rs1 = state.regs.read(self.rs1)?;
        let addr = (rs1 + self.imm).try_into()?;
        let data = state.imem.read_byte(addr)?;
        let result = if data & 0x80 != 0 {
            (data as u32 | 0xffff_ff00) as i32
        } else {
            data as i32
        };
        state.regs.write(self.rd, result)?;
        state.add_trace(InstType::IMemRead, Some(addr), Some(result), Some(self.rd));
        state.pc += 6;
        Ok(state)
    }
}
