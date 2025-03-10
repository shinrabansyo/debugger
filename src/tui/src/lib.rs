#![feature(gen_blocks)]

use sb_emu::State as EmuState;

use sb_dbg_tui_engine::workspace::WorkspaceBuilder;
use sb_dbg_tui_engine::UI;
use sb_dbg_tui_widget_inst::Inst;
use sb_dbg_tui_widget_device_gpout::Gpout;
use sb_dbg_tui_widget_device_uart::Uart;
use sb_dbg_tui_widget_reg::Register;
use sb_dbg_tui_widget_mem::Mem;

pub fn run(pc: u32, dmem: &[u8], imem: &[u8]) -> anyhow::Result<()> {
    let emu = EmuState::new(pc, &dmem, &imem);

    let workspaces = [
        WorkspaceBuilder::default()
            .name("Workspace 0")
            .widget((0, 0), Box::new(Inst::default()))
            .widget((0, 1), Box::new(Uart::default()))
            .widget((1, 0), Box::new(Register::default()))
            .widget((1, 1), Box::new(Mem::default()))
            .build(),
        WorkspaceBuilder::default()
            .name("Workspace 1")
            .widget((0, 0), Box::new(Inst::default()))
            .widget((0, 1), Box::new(Gpout::default()))
            .widget((1, 0), Box::new(Register::default()))
            .widget((1, 1), Box::new(Mem::default()))
            .build(),
    ];

    UI::new(emu, workspaces).run(&mut ratatui::init())?;
    ratatui::restore();

    Ok(())
}
