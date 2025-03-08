use sb_dbg_utils::setup_from_args;

fn main() -> anyhow::Result<()> {
    let (pc, dmem, imem) = setup_from_args()?;
    sb_dbg_tui::run(pc, &dmem, &imem)
}
