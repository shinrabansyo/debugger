use super::Device;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Timer {
    clock_counter: u64,   // read only
    wakeup_time: Instant, // read only
    interrupt_time: u64,  // read/write
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            clock_counter: 0,
            wakeup_time: Instant::now(),
            interrupt_time: 0,
        }
    }
}

impl Device for Timer {
    fn read(&self, addr: usize) -> anyhow::Result<u32> {
        match addr {
            0x0000_0400 => Ok((self.clock_counter & 0xFFFF_FFFF) as u32),
            0x0000_0401 => Ok((self.clock_counter >> 32 & 0xFFFF_FFFF) as u32),
            0x0000_0402 => Ok(0),
            0x0000_0403 => Ok((self.wakeup_time.elapsed().as_millis() & 0xFFFF_FFFF) as u32), // リセット後からの経過時間（ミリ秒）[31:0]
            0x0000_0404 => {
                Ok(((self.wakeup_time.elapsed().as_millis() >> 32) & 0xFFFF_FFFF) as u32)
            } // リセット後からの経過時間（ミリ秒）[63:32]
            0x0000_0405 => Ok((self.interrupt_time & 0xFFFF_FFFF) as u32), // 経過時間割込み（ミリ秒）[31:0]
            0x0000_0406 => Ok((self.interrupt_time >> 32 & 0xFFFF_FFFF) as u32), // 経過時間割込み（ミリ秒）[63:32]
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }

    fn write(&mut self, addr: usize, data: u32) -> anyhow::Result<()> {
        match addr {
            0x0000_0405 => {
                self.interrupt_time = (self.interrupt_time & 0xFFFF_FFFF_0000_0000) | data as u64;
                Ok(())
            } // 経過時間割込み（ミリ秒）[31:0]
            0x0000_0406 => {
                self.interrupt_time =
                    (self.interrupt_time & 0x0000_0000_FFFF_FFFF) | ((data as u64) << 32);
                Ok(())
            } // 経過時間割込み（ミリ秒）[63:32]
            _ => Err(anyhow::anyhow!("Invalid device addr: 0x{:08x}", addr)),
        }
    }
}

impl Timer {
    pub fn tick(&mut self) {
        self.clock_counter += 1;
    }

    pub fn check_interrupt(&self) -> bool {
        let elapsed_millis = self.wakeup_time.elapsed().as_millis() as u64;
        elapsed_millis == self.interrupt_time
    }
}
