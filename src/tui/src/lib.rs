#![feature(gen_blocks)]

mod ui;

use sb_emu::State as EmuState;

use ui::workspace::WorkspaceBuilder;
use ui::{widget, UI};

pub fn run(pc: u32, dmem: &[u8], imem: &[u8]) -> anyhow::Result<()> {
    let emu = EmuState::new(pc, &dmem, &imem);

    let workspace_0 = WorkspaceBuilder::default()
        .widget((0, 0), Box::new(widget::inst::InstState::default()))
        .widget((0, 1), Box::new(widget::device::DeviceState::default()))
        .widget((1, 0), Box::new(widget::reg::RegisterState::default()))
        .widget((1, 1), Box::new(widget::mem::MemState::default()))
        .build();
    let workspaces = [workspace_0];

    UI::new(emu, workspaces).run(&mut ratatui::init())?;
    ratatui::restore();

    Ok(())
}
