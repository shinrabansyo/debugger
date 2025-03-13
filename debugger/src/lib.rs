use std::ops::{Deref, DerefMut};

use sb_emu::Emulator;

pub struct Debugger {
    emu: Emulator,
}

impl From<Emulator> for Debugger {
    fn from(emu: Emulator) -> Debugger {
        Debugger { emu }
    }
}

impl Deref for Debugger {
    type Target = Emulator;

    fn deref(&self) -> &Emulator {
        &self.emu
    }
}

impl DerefMut for Debugger {
    fn deref_mut(&mut self) -> &mut Emulator {
        &mut self.emu
    }
}

impl Debugger {
    pub fn run_until_break(&mut self) -> anyhow::Result<()> {
        // TODO
        loop {
            let inst = self.imem.read::<6>(self.pc as usize)?;
            if inst == 0x0000_0003 {
                return Ok(());
            }
            self.step()?;
        }
    }
}
