mod display;
mod gpio;
mod uart;

use image::DynamicImage;

use uart::Uart;
use gpio::Gpio;
use display::Display;

pub(super) trait Device {
    fn read(&self, addr: usize) -> anyhow::Result<u32>;
    fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, Default)]
pub struct DeviceMap {
    uart: Uart,
    gpio: Gpio,
    display: Display,
}

impl DeviceMap {
    pub fn read(&self, addr: usize) -> anyhow::Result<u32> {
        match addr {
            0x0000_0000 => self.uart.read(addr),
            0x0000_0004 => self.gpio.read(addr),
            0x0000_0006..=0x0000_0007 => self.display.read(addr),
            0x1000_0000..=0x1000_ffff => self.display.read(addr),
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }

    pub fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()> {
        match addr {
            0x0000_0000 => self.uart.write(addr, data),
            0x0000_0004 => self.gpio.write(addr, data),
            0x0000_0006..=0x0000_0007 => self.display.write(addr, data),
            0x1000_0000..=0x1000_ffff => self.display.write(addr, data),
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }

    pub fn get_uart_stat(&self) -> &str {
        self.uart.get_stat()
    }

    pub fn get_gpio_stat(&self) -> u8 {
        self.gpio.get_stat()
    }

    pub fn get_display_stat(&self) -> ((u32, u32), DynamicImage) {
        self.display.get_stat()
    }
}
