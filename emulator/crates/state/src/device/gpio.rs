use super::Device;

#[derive(Debug, Clone, Default)]
pub struct Gpio {
    state: u8,
}

impl Device for Gpio {
    fn read(&self, _: usize) -> anyhow::Result<u32> {
        Ok(self.state as u32)
    }

    fn write(&mut self, _: usize, data: u32) -> anyhow::Result<()> {
        self.state = (data & 0xff) as u8;
        Ok(())
    }
}

impl Gpio {
    pub fn get_stat(&self) -> u8 {
        self.state
    }
}
