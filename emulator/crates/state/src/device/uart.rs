#[derive(Debug, Clone)]
pub struct Uart;

impl Uart {
    pub fn new() -> Uart {
        Uart
    }

    pub fn read(&self, _: usize) -> anyhow::Result<u32> {
        Ok(0)
    }

    pub fn write(&mut self, _: usize, data: u32) -> anyhow::Result<()> {
        let c = (data & 0xff) as u8;
        print!("{}", c as char);
        Ok(())
    }
}
