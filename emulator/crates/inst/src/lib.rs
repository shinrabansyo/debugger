mod def;

use sb_emu_state::State;

pub use def::*;

pub trait Inst {
    fn exec(&self, state: State) -> anyhow::Result<State>;
}
