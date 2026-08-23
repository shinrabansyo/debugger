mod device;
mod mem;
mod reg;

pub use device::interrupt::InterruptCause;
pub use device::DeviceMap;
pub use mem::Memory;
pub use reg::Registers;

#[derive(Debug, Clone, Copy)]
pub enum InstType {
    RegWrite,
    MemRead,
    MemWrite,
    IMemRead,
    IMemWrite,
    DevRead,
    DevWrite,
}

#[derive(Debug, Clone, Copy)]
pub struct InstLog {
    pub pc: u32,
    pub instr: u64,
    pub inst_type: InstType,
    pub addr: Option<usize>,
    pub data: Option<i32>,
    pub dest_reg: Option<usize>,
}

impl InstLog {
    pub fn new(
        state: &State,
        inst_type: InstType,
        addr: Option<usize>,
        data: Option<i32>,
        dest_reg: Option<usize>,
    ) -> Self {
        InstLog {
            pc: state.pc,
            instr: state.imem.read::<6>(state.pc as usize).unwrap(),
            inst_type,
            addr,
            data,
            dest_reg,
        }
    }
}

impl From<InstLog> for String {
    fn from(log: InstLog) -> Self {
        // from Veryl
        // $sformatf("PC:%08h INST:%012h TYPE:%s ADDR:%08h VAL:%08h, DST:%02h", log.pc, log.instr, type_str, log.addr, log.data);
        let type_str = match log.inst_type {
            InstType::RegWrite => "R_WR",
            InstType::MemRead => "M_RD",
            InstType::MemWrite => "M_WR",
            InstType::IMemRead => "I_RD",
            InstType::IMemWrite => "I_WR",
            InstType::DevRead => "D_RD",
            InstType::DevWrite => "D_WR",
            // _ => "UNKN",
        };
        format!(
            "PC:{:08x} INST:{:012x} TYPE:{} ADDR:{:08x} VAL:{:08x}, DST:{:02}\n",
            log.pc,
            log.instr,
            type_str,
            log.addr.unwrap_or(0),
            log.data.unwrap_or(0),
            log.dest_reg.unwrap_or(0)
        )
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub pc: u32,
    pub regs: Registers,
    pub dmem: Memory<{ 1 * 1024 * 1024 }>,
    pub imem: Memory<{ 1 * 1024 * 1024 }>,
    pub devices: DeviceMap,
    pub last_result: Option<InstLog>,
}

impl State {
    pub fn new(pc: u32, dmem: &[u8], imem: &[u8]) -> Self {
        State {
            pc,
            regs: Registers::new(),
            dmem: Memory::from(dmem),
            imem: Memory::from(imem),
            devices: DeviceMap::default(),
            last_result: None,
        }
    }
    pub fn add_trace(
        &mut self,
        inst_type: InstType,
        addr: Option<usize>,
        data: Option<i32>,
        dest_reg: Option<usize>,
    ) {
        let log = InstLog::new(self, inst_type, addr, data, dest_reg);
        self.last_result = Some(log);
    }
}
