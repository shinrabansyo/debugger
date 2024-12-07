use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Gpio {
    state: u8,
}

impl Gpio {
    pub fn new() -> Gpio {
        Gpio {
            state: 0,
        }
    }

    pub fn read(&self, _: usize) -> anyhow::Result<u32> {
        Ok(self.state as u32)
    }

    pub fn write(&mut self, _: usize, data: u32) -> anyhow::Result<()> {
        self.state = (data & 0xff) as u8;
        Ok(())
    }

    pub fn get_stat(&self, _: usize) -> anyhow::Result<String> {
        let mut line_1 = String::new();
        let mut line_2 = String::new();
        for idx in 0..8 {
            let pin = 7 - idx;
            write!(line_1, " [{}] ", pin)?;
            if self.state & (1 << pin) != 0 {
                write!(line_2, "  O  ")?;
            } else {
                write!(line_2, "  _  ")?;
            }
        }
        Ok(format!("{}\n{}", line_1, line_2))
    }
}
