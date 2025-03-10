#![feature(gen_blocks)]

use sb_emu::State as EmuState;

use sb_dbg_tui_engine::workspace::WorkspaceBuilder;
use sb_dbg_tui_engine::UI;
use sb_dbg_tui_widget_inst::InstState;
use sb_dbg_tui_widget_device::DeviceState;
use sb_dbg_tui_widget_reg::RegisterState;
use sb_dbg_tui_widget_mem::MemState;

pub fn run(pc: u32, dmem: &[u8], imem: &[u8]) -> anyhow::Result<()> {
    let emu = EmuState::new(pc, &dmem, &imem);

    let workspace_0 = WorkspaceBuilder::default()
        .widget((0, 0), Box::new(InstState::default()))
        .widget((0, 1), Box::new(DeviceState::default()))
        .widget((1, 0), Box::new(RegisterState::default()))
        .widget((1, 1), Box::new(MemState::default()))
        .build();
    let workspaces = [workspace_0];

    UI::new(emu, workspaces).run(&mut ratatui::init())?;
    ratatui::restore();

    Ok(())
}
