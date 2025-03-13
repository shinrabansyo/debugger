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
