mod ui;

use sb_emu::State as EmuState;

use ui::UI;

fn main() -> anyhow::Result<()> {
    let init_state = EmuState::new(0, &[0], &[0]);

    let mut terminal = ratatui::init();
    UI::new(init_state).run(&mut terminal)?;
    ratatui::restore();

    Ok(())
}
