use sb_emu_inst::parse;
pub use sb_emu_state::State;

pub fn step(state: State) -> anyhow::Result<State> {
    let raw_inst = state.imem.read::<6>(state.pc as usize)?;
    let inst = parse(raw_inst)?;
    inst.exec(state)
}
