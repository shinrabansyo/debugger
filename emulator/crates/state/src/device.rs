mod display;
mod gpio;
pub mod interrupt;
mod timer;
mod uart;

use image::DynamicImage;

use display::Display;
use gpio::Gpio;
use interrupt::InterruptController;
use timer::Timer;
use uart::Uart;

pub(super) trait Device {
    fn read(&self, addr: usize) -> anyhow::Result<u32>;
    fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, Default)]
pub struct DeviceMap {
    uart: Uart,
    gpio: Gpio,
    pub interrupt: InterruptController,
    display: Display,
    pub timer: Timer,
}

impl DeviceMap {
    pub fn read(&self, addr: usize) -> anyhow::Result<u32> {
        match addr {
            0x0000_0000..0x0000_0100 => self.uart.read(addr),
            0x0000_0100..0x0000_0200 => unimplemented!("SPI is not implemented yet"),
            0x0000_0200..0x0000_0300 => self.gpio.read(addr),
            0x0000_0300..0x0000_0400 => self.display.read(addr),
            0x0000_0400..0x0000_0500 => unimplemented!("Clock Counter"),
            0x0000_0500..0x0000_0600 => self.interrupt.read(addr),
            0x0100_0000..0x01ff_ffff => self.display.read(addr),
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }

    pub fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()> {
        match addr {
            0x0000_0000..0x0000_0100 => self.uart.write(addr, data),
            0x0000_0100..0x0000_0200 => unimplemented!("SPI is not implemented yet"),
            0x0000_0200..0x0000_0300 => self.gpio.write(addr, data),
            0x0000_0300..0x0000_0400 => self.display.write(addr, data),
            0x0000_0400..0x0000_0500 => unimplemented!("Clock Counter"),
            0x0000_0500..0x0000_0600 => self.interrupt.write(addr, data),
            0x0100_0000..0x01ff_ffff => self.display.write(addr, data),
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
