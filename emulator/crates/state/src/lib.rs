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
    pub dmem: Memory<{1024 * 4}>,
    pub imem: Memory<{1024 * 6}>,
    pub devices: DeviceMap,
}

impl State {
    pub fn new(pc: u32, dmem: &[u8], imem: &[u8]) -> Self {
        State {
            pc,
            regs: Registers::new(),
            dmem: Memory::from(dmem),
            imem: Memory::from(imem),
            devices: DeviceMap::default(),
        }
    }
}
