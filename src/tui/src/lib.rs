mod ui;

use sb_emu::State as EmuState;

use ui::UI;

pub fn run(pc: u32, dmem: &[u8], imem: &[u8]) -> anyhow::Result<()> {
    let init_state = EmuState::new(pc, &dmem, &imem);

    let mut terminal = ratatui::init();
    UI::new(init_state).run(&mut terminal)?;
    ratatui::restore();

    Ok(())
}
