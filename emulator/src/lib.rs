use std::ops::Deref;

use sb_emu_inst::parse;
use sb_emu_state::State;

pub use sb_emu_state::{Registers, Memory, DeviceMap};

#[derive(Debug, Clone)]
pub struct Emulator {
    state: Option<State>,
}

impl Deref for Emulator {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        self.state.as_ref().unwrap()
    }
}

impl Emulator {
    pub fn new(pc: u32, dmem: &[u8], imem: &[u8]) -> Self {
        Emulator {
            state: Some(State::new(pc, dmem, imem)),
        }
    }

    pub fn step(&mut self) -> anyhow::Result<()> {
        let state = self.state.take().unwrap();
        let raw_inst = state.imem.read::<6>(state.pc as usize)?;
        self.state = Some(parse(raw_inst)?.exec(state)?);
        Ok(())
    }
}
