mod opts;
mod load;
mod ui;

use sb_emu::State as EmuState;

use opts::{parse_args, FileFormat};
use load::{load_assembly, load_bytechar, load_bytechar2};
use ui::UI;

pub fn run() -> anyhow::Result<()> {
    let options = parse_args();

    // エミュレータ初期化
    let (dmem, imem) = match (options.format, options.data) {
        (FileFormat::Assembly, _) => load_assembly(&options.inst)?,
        (FileFormat::ByteChar, Some(data)) => load_bytechar2(&data, &options.inst)?,
        (FileFormat::ByteChar, None) => (load_bytechar(&options.inst)?, vec![]),
    };
    let init_state = EmuState::new(options.pc, &dmem, &imem);

    // TUI 起動
    let mut terminal = ratatui::init();
    UI::new(init_state).run(&mut terminal)?;
    ratatui::restore();

    Ok(())
}
