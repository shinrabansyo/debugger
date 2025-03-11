mod uart;
mod gpio;

use uart::Uart;
use gpio::Gpio;

pub(super) trait Device {
    fn read(&self, addr: usize) -> anyhow::Result<u32>;
    fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, Default)]
pub struct DeviceMap {
    uart: Uart,
    gpio: Gpio,
}

impl DeviceMap {
    pub fn read(&self, addr: usize) -> anyhow::Result<u32> {
        match addr {
            0x0000_0000 => self.uart.read(addr),
            0x0000_0004 => self.gpio.read(addr),
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }

    pub fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()> {
        match addr {
            0x0000_0000 => self.uart.write(addr, data),
            0x0000_0004 => self.gpio.write(addr, data),
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }

    pub fn get_uart_stat(&self) -> &str {
        self.uart.get_stat()
    }

    pub fn get_gpio_stat(&self) -> u8 {
        self.gpio.get_stat()
    }
}
