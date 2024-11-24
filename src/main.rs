mod ui;

use ui::UI;

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    UI::new().run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
