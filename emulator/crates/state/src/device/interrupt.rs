use super::Device;

#[derive(Debug, Clone, Default)]
pub struct InterruptController {
    pub trap_vector: usize,
    pub previous_pc: usize,
    pub interrupt_cause: u32,
    pub interrupt_enable: u32,
}

pub enum InterruptCause {
    Timer = 1,
}

impl Device for InterruptController {
    fn read(&self, addr: usize) -> anyhow::Result<u32> {
        match addr {
            0x0000_0500 => Ok(self.trap_vector as u32),
            0x0000_0501 => Ok(self.previous_pc as u32),
            0x0000_0502 => Ok(self.interrupt_cause),
            0x0000_0503 => Ok(self.interrupt_enable),
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }

    fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()> {
        match addr {
            0x0000_0500 => {
                self.trap_vector = data as usize;
                Ok(())
            }
            0x0000_0501 => {
                self.previous_pc = data as usize;
                Ok(())
            }
            0x0000_0502 => {
                self.interrupt_cause = data;
                Ok(())
            }
            0x0000_0503 => {
                self.interrupt_enable = data;
                Ok(())
            }
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }
}

impl InterruptController {
    pub fn get_gie(&self) -> bool {
        (self.interrupt_enable & 0x01) != 0
    }

    pub fn set_gie(&mut self, value: bool) {
        if value {
            self.interrupt_enable |= 0x01;
        } else {
            self.interrupt_enable &= !0x01;
        }
    }

    pub fn get_tie(&self) -> bool {
        (self.interrupt_enable & 0x02) != 0
    }

    pub fn set_tie(&mut self, value: bool) {
        if value {
            self.interrupt_enable |= 0x02;
        } else {
            self.interrupt_enable &= !0x02;
        }
    }

    pub fn get_uie(&self) -> bool {
        (self.interrupt_enable & 0x04) != 0
    }

    pub fn set_uie(&mut self, value: bool) {
        if value {
            self.interrupt_enable |= 0x04;
        } else {
            self.interrupt_enable &= !0x04;
        }
    }

    pub fn set_sie(&mut self, value: bool) {
        if value {
            self.interrupt_enable |= 0x08;
        } else {
            self.interrupt_enable &= !0x08;
        }
    }

    pub fn set_vie(&mut self, value: bool) {
        if value {
            self.interrupt_enable |= 0x10;
        } else {
            self.interrupt_enable &= !0x10;
        }
    }
}
