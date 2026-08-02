use super::Device;

#[derive(Debug, Clone, Default)]
pub struct Uart {
    out: String,
}

impl Device for Uart {
    fn read(&self, addr: usize) -> anyhow::Result<u32> {
        match addr {
            0x0000_0000 => Ok(0),    // Receiver Holding Register
            0x0000_0005 => Ok(0x20), // Line Status Register: THR Empty, Data not Ready
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }

    fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()> {
        match addr {
            // Transmitter Holding Register
            0x0000_0000 => {
                let c = (data & 0xff) as u8;
                self.out.push(c as char);
                Ok(())
            }
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }
}

impl Uart {
    pub fn get_stat(&self) -> &str {
        &self.out
    }
}
