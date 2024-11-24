mod reg;
mod mem;
mod device;

use reg::Registers;
use mem::Memory;
use device::DeviceMap;

#[derive(Debug, Clone)]
pub struct State {
    pub pc: u32,
    pub regs: Registers,
    pub imem: Memory<{1024 * 6}>,
    pub dmem: Memory<{1024 * 4}>,
    pub devices: DeviceMap,
}

impl State {
    pub fn new() -> Self {
        State {
            pc: 0,
            regs: Registers::new(),
            imem: Memory::new(),
            dmem: Memory::new(),
            devices: DeviceMap::new(),
        }
    }
}
