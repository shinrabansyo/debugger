mod reg;
mod mem;
mod device;

pub use reg::Registers;
pub use mem::Memory;
pub use device::DeviceMap;

#[derive(Debug, Clone)]
pub struct State {
    pub pc: u32,
    pub regs: Registers,
    pub dmem: Memory<{1 * 1024 * 1024}>,
    pub imem: Memory<{1 * 1024 * 1024}>,
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
