mod opts;
mod load;
mod ui;

use sb_emu::State as EmuState;

use opts::{parse_args, FileFormat};
use load::load_bytechar;
use ui::UI;

fn main() -> anyhow::Result<()> {
    let options = parse_args();

    let dmem = match (options.data, options.format) {
        (Some(data), FileFormat::ByteChar) => load_bytechar(&data)?,
        _ => vec![],
    };
    let imem = match options.format {
        FileFormat::ByteChar => load_bytechar(&options.inst)?,
    };
    let init_state = EmuState::new(options.pc, &dmem, &imem);

    let mut terminal = ratatui::init();
    UI::new(init_state).run(&mut terminal)?;
    ratatui::restore();

    Ok(())
}
