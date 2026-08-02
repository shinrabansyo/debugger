use super::Device;

#[derive(Debug, Clone, Default)]
pub struct Gpio {
    state: u8,
}

impl Device for Gpio {
    fn read(&self, addr: usize) -> anyhow::Result<u32> {
        match addr {
            0x0000_0200 => Ok(self.state as u32), // GPOut
            0x0000_0201 => Ok(0),                 // GPIn
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }

    fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()> {
        match addr {
            // GPOut
            0x0000_0200 => {
                self.state = (data & 0xff) as u8;
                Ok(())
            }
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }
}

impl Gpio {
    pub fn get_stat(&self) -> u8 {
        self.state
    }
}
