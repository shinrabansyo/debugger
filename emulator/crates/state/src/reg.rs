#[derive(Debug, Clone)]
pub struct Registers {
    regs: [u32; 32],
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            regs: [0; 32],
        }
    }

    pub fn read(&self, reg: usize) -> anyhow::Result<u32> {
        self.regs
            .get(reg)
            .map(|&val| val)
            .ok_or(anyhow::anyhow!("Invalid register number : {}", reg))
    }

    pub fn write(&mut self, reg: usize, val: u32) -> anyhow::Result<()> {
        match reg {
            0 => Ok(()),
            1..=31 => {
                self.regs[reg] = val;
                Ok(())
            }
            _ => return Err(anyhow::anyhow!("Invalid register number : {}", reg)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn registers_ok() {
        let mut regs = Registers::new();

        assert_eq!(regs.read(0).unwrap(), 0);
        assert_eq!(regs.read(1).unwrap(), 0);

        assert!(regs.write(0, 1).is_ok());
        assert!(regs.write(1, 2).is_ok());

        assert_eq!(regs.read(0).unwrap(), 0);
        assert_eq!(regs.read(1).unwrap(), 2);
    }

    #[test]
    fn registers_bound() {
        let mut regs = Registers::new();

        assert!(regs.read(31).is_ok());
        assert!(regs.read(32).is_err());

        assert!(regs.write(31, 0).is_ok());
        assert!(regs.write(32, 0).is_err());
    }
}
