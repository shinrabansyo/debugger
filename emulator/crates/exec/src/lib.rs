mod state;

use state::State;

#[derive(Debug, Clone)]
pub struct Executor {
    state: State,
}

impl Executor {
    pub fn exec() -> anyhow::Result<()> {
        Ok(())
    }
}

pub trait Execulatble {
    fn exec(&self, state: &mut State) -> anyhow::Result<()>;
}
