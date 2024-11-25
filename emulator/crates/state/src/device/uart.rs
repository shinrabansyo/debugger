#[derive(Debug, Clone)]
pub struct Uart {
    out: String,
}

impl Uart {
    pub fn new() -> Uart {
        Uart {
            out: String::new(),
        }
    }

    pub fn read(&self, _: usize) -> anyhow::Result<u32> {
        Ok(0)
    }

    pub fn write(&mut self, _: usize, data: u32) -> anyhow::Result<()> {
        let c = (data & 0xff) as u8;
        self.out.push(c as char);
        Ok(())
    }

    pub fn get_stat(&self, _: usize) -> anyhow::Result<String> {
        Ok(self.out.clone())
    }
}
