#[derive(Debug, Clone)]
pub struct Memory<const SIZE: usize> {
    pub mem: Vec<u8>,
}

impl<const SIZE: usize> From<&[u8]> for Memory<SIZE> {
    fn from(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= SIZE);

        let mut mem = vec![0; SIZE];
        for (idx, byte) in bytes.iter().enumerate() {
            mem[idx] = *byte;
        }
        Memory { mem }
    }
}

impl<const SIZE: usize> Memory<SIZE> {
    pub fn new() -> Self {
        Memory {
            mem: vec![0; SIZE],
        }
    }

    pub fn read_byte(&self, addr: usize) -> anyhow::Result<u8> {
        self.read::<1>(addr).map(|val| val as u8)
    }

    pub fn read_half(&self, addr: usize) -> anyhow::Result<u16> {
        self.read::<2>(addr).map(|val| val as u16)
    }

    pub fn read_word(&self, addr: usize) -> anyhow::Result<u32> {
        self.read::<4>(addr).map(|val| val as u32)
    }

    fn read<const WIDTH: usize>(&self, addr: usize) -> anyhow::Result<u64> {
        Self::bound_check(addr, WIDTH)?;

        let local_mem = &self.mem[addr..addr+WIDTH];
        let mut result = 0;
        for idx in 0..WIDTH {
            result |= (local_mem[idx] as u64) << (8 * idx);
        }
        Ok(result)
    }

    pub fn write_byte(&mut self, addr: usize, value: u8) -> anyhow::Result<()> {
        self.write::<1>(addr, value as u64)
    }

    pub fn write_half(&mut self, addr: usize, value: u16) -> anyhow::Result<()> {
        self.write::<2>(addr, value as u64)
    }

    pub fn write_word(&mut self, addr: usize, value: u32) -> anyhow::Result<()> {
        self.write::<4>(addr, value as u64)
    }

    fn write<const WIDTH: usize>(&mut self, addr: usize, value: u64) -> anyhow::Result<()> {
        Self::bound_check(addr, WIDTH)?;

        for idx in 0..WIDTH {
            self.mem[addr+idx] = (value >> (8 * idx)) as u8;
        }
        Ok(())
    }

    fn bound_check(begin: usize, len: usize) -> anyhow::Result<()> {
        if begin + len > SIZE {
            Err(anyhow::anyhow!("Out of bound memory access : [{}..{})", begin, begin+len))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn memory_byte_ok() {
        const SIZE: usize = 8;
        let mut mem = Memory::<SIZE>::new();

        /*  0: 'h'
            1: 'e'
            2: 'l'
            3: 'l'
            4: 'o'
        */

        assert!(mem.write_byte(0, 'h' as u8).is_ok());
        assert!(mem.write_byte(1, 'e' as u8).is_ok());
        assert!(mem.write_byte(2, 'l' as u8).is_ok());
        assert!(mem.write_byte(3, 'l' as u8).is_ok());
        assert!(mem.write_byte(4, 'o' as u8).is_ok());

        assert_eq!(mem.read_byte(0).unwrap(), 'h' as u8);
        assert_eq!(mem.read_byte(1).unwrap(), 'e' as u8);
        assert_eq!(mem.read_byte(2).unwrap(), 'l' as u8);
        assert_eq!(mem.read_byte(3).unwrap(), 'l' as u8);
        assert_eq!(mem.read_byte(4).unwrap(), 'o' as u8);
    }

    #[test]
    fn memory_byte_bound() {
        const SIZE: usize = 8;
        let mut mem = Memory::<SIZE>::new();

        assert!(mem.read_byte(7).is_ok());
        assert!(mem.read_byte(8).is_err());

        assert!(mem.write_byte(7, 'h' as u8).is_ok());
        assert!(mem.write_byte(8, 'w' as u8).is_err());
    }

    #[test]
    fn memory_half() {
        const SIZE: usize = 8;
        let mut mem = Memory::<SIZE>::new();

        /*  0: 0x34
            1: 0x12
            2: 0x78
            3: 0x56
         */

        assert!(mem.write_half(0, 0x1234).is_ok());
        assert!(mem.write_half(2, 0x5678).is_ok());

        assert_eq!(mem.read_byte(0).unwrap(), 0x34);
        assert_eq!(mem.read_byte(1).unwrap(), 0x12);
        assert_eq!(mem.read_byte(2).unwrap(), 0x78);
        assert_eq!(mem.read_byte(3).unwrap(), 0x56);

        assert_eq!(mem.read_half(0).unwrap(), 0x1234);
        assert_eq!(mem.read_half(1).unwrap(), 0x7812);
        assert_eq!(mem.read_half(2).unwrap(), 0x5678);
    }

    #[test]
    fn memory_half_bound() {
        const SIZE: usize = 8;
        let mut mem = Memory::<SIZE>::new();

        assert!(mem.read_half(6).is_ok());
        assert!(mem.read_half(7).is_err());
        assert!(mem.read_half(8).is_err());

        assert!(mem.write_half(6, 0x1234).is_ok());
        assert!(mem.write_half(7, 0x1234).is_err());
        assert!(mem.write_half(8, 0x1234).is_err());
    }

    #[test]
    fn memory_word() {
        const SIZE: usize = 8;
        let mut mem = Memory::<SIZE>::new();

        /*  0: 0x78
            1: 0x56
            2: 0x34
            3: 0x12
            4: 0x21
            5: 0x43
            6: 06x5
            7: 0x87
         */

        assert!(mem.write_word(0, 0x12345678).is_ok());
        assert!(mem.write_word(4, 0x87654321).is_ok());

        assert_eq!(mem.read_byte(0).unwrap(), 0x78);
        assert_eq!(mem.read_byte(1).unwrap(), 0x56);
        assert_eq!(mem.read_byte(2).unwrap(), 0x34);
        assert_eq!(mem.read_byte(3).unwrap(), 0x12);
        assert_eq!(mem.read_byte(4).unwrap(), 0x21);
        assert_eq!(mem.read_byte(5).unwrap(), 0x43);
        assert_eq!(mem.read_byte(6).unwrap(), 0x65);
        assert_eq!(mem.read_byte(7).unwrap(), 0x87);

        assert_eq!(mem.read_word(0).unwrap(), 0x12345678);
        assert_eq!(mem.read_word(1).unwrap(), 0x21123456);
        assert_eq!(mem.read_word(2).unwrap(), 0x43211234);
        assert_eq!(mem.read_word(3).unwrap(), 0x65432112);
        assert_eq!(mem.read_word(4).unwrap(), 0x87654321);
    }

    #[test]
    fn memory_word_bound() {
        const SIZE: usize = 8;
        let mut mem = Memory::<SIZE>::new();

        assert!(mem.read_word(4).is_ok());
        assert!(mem.read_word(5).is_err());
        assert!(mem.read_word(6).is_err());
        assert!(mem.read_word(7).is_err());
        assert!(mem.read_word(8).is_err());

        assert!(mem.write_word(4, 0x12345678).is_ok());
        assert!(mem.write_word(5, 0x12345678).is_err());
        assert!(mem.write_word(6, 0x12345678).is_err());
        assert!(mem.write_word(7, 0x12345678).is_err());
        assert!(mem.write_word(8, 0x12345678).is_err());
    }
}
