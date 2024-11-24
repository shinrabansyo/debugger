mod uart;

use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::RwLock;

use uart::Uart;

#[derive(Debug, Clone)]
pub struct DeviceMap {
    map: HashMap<usize, Rc<RwLock<Device>>>,
}

impl DeviceMap {
    pub fn new() -> DeviceMap {
        let uart = Rc::new(RwLock::new(Device::Uart(Uart::new())));

        DeviceMap {
            map: HashMap::from([
                (0x0000_0000, uart),
            ]),
        }
    }

    pub fn read(&self, addr: usize) -> anyhow::Result<u32> {
        if let Some(device) = self.map.get(&addr) {
            device.read().unwrap().read(addr)
        } else {
            Err(anyhow::anyhow!("Device addr {} is not registered", addr))
        }
    }

    pub fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()> {
        if let Some(device) = self.map.get_mut(&addr) {
            device.write().unwrap().write(addr, data)
        } else {
            Err(anyhow::anyhow!("Device addr {} is not registered", addr))
        }
    }
}

#[derive(Debug, Clone)]
enum Device {
    Uart(Uart),
}

impl Device {
    pub fn read(&self, addr: usize) -> anyhow::Result<u32> {
        match self {
            Device::Uart(uart) => uart.read(addr),
        }
    }

    pub fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()> {
        match self {
            Device::Uart(uart) => uart.write(addr, data),
        }
    }
}
