use super::Device;

#[derive(Debug, Clone, Default)]
pub struct Uart {
    out: String,
}

impl Device for Uart {
    fn read(&self, _: usize) -> anyhow::Result<u32> {
        Ok(0)
    }

    fn write(&mut self, _: usize, data: u32) -> anyhow::Result<()> {
        let c = (data & 0xff) as u8;
        self.out.push(c as char);
        Ok(())
    }

    fn get_stat(&self, _: usize) -> anyhow::Result<String> {
        Ok(self.out.clone())
    }
}
