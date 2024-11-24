mod reg;
mod mem;

use reg::Registers;
use mem::Memory;

#[derive(Debug, Clone)]
pub struct State {
    pub regs: Registers,
    pub imem: Memory<{1024 * 6}>,
    pub dmem: Memory<{1024 * 4}>,
}

impl State {
    pub fn new() -> Self {
        State {
            regs: Registers::new(),
            imem: Memory::new(),
            dmem: Memory::new(),
        }
    }
}
