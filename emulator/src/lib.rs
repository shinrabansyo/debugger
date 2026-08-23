use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Deref;
use std::path::PathBuf;

use sb_emulator_inst::parse;
use sb_emulator_state::State;

pub use sb_emulator_state::{DeviceMap, Memory, Registers};

#[derive(Debug)]
struct TraceDumper {
    writer: BufWriter<File>,
}
impl TraceDumper {
    fn new(path: &PathBuf) -> anyhow::Result<Self> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(b"")?;
        Ok(TraceDumper { writer })
    }

    fn write(&mut self, content: &str) -> anyhow::Result<()> {
        self.writer.write_all(content.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Emulator {
    state: Option<State>,
    tracer: Option<TraceDumper>,
}

impl Deref for Emulator {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        self.state.as_ref().unwrap()
    }
}

impl Emulator {
    pub fn new(pc: u32, dmem: &[u8], imem: &[u8], trace_path: Option<PathBuf>) -> Self {
        Emulator {
            state: Some(State::new(pc, dmem, imem)),
            tracer: match trace_path {
                Some(path) => Some(TraceDumper::new(&path).unwrap()),
                None => None,
            },
        }
    }

    pub fn step(&mut self) -> anyhow::Result<()> {
        let mut state = self.state.take().unwrap();

        // check interrupt
        if state.devices.interrupt.get_gie() {
            if state.devices.interrupt.get_tie() && state.devices.timer.check_interrupt() {
                // interrupt
                state.devices.interrupt.previous_pc = state.pc as usize;
                state.pc = state.devices.interrupt.trap_vector as u32;
                state.devices.interrupt.interrupt_cause = 0x01; // timer interrupt
                state.devices.interrupt.interrupt_enable &= !0x01; // disable interrupt
                return Ok(());
            }
        }

        let raw_inst = state.imem.read::<6>(state.pc as usize)?;
        self.state = Some(parse(raw_inst)?.exec(state)?);
        if let Some(tracer) = &mut self.tracer {
            tracer.write(&String::from(
                self.state.as_ref().unwrap().last_result.unwrap(),
            ))?;
        }
        if let Some(state) = &mut self.state {
            state.devices.timer.tick();
        }
        Ok(())
    }
}
