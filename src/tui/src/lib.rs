#![feature(gen_blocks)]

use sb_emu::State as EmuState;

use sb_dbg_tui_engine::widget::Widget;
use sb_dbg_tui_engine::workspace::WorkspaceBuilder;
use sb_dbg_tui_engine::UI;
use sb_dbg_tui_widget_inst::Inst;
use sb_dbg_tui_widget_device_display::Display;
use sb_dbg_tui_widget_device_gpout::Gpout;
use sb_dbg_tui_widget_device_uart::Uart;
use sb_dbg_tui_widget_reg::Register;
use sb_dbg_tui_widget_mem::Mem;

pub fn run(pc: u32, dmem: &[u8], imem: &[u8]) -> anyhow::Result<()> {
    let emu = EmuState::new(pc, &dmem, &imem);

    let inst_widget = Inst::new();
    let reg_widget = Register::new();
    let mem_widget = Mem::new();
    let gpout_widget = Gpout::new();
    let uart_widget = Uart::new();
    let display_widget = Display::<128, 128>::new();

    let workspaces = [
        WorkspaceBuilder::default()
            .name("Workspace 0")
            .widget((0, 0), &inst_widget)
            .widget((0, 1), &uart_widget)
            .widget((1, 0), &reg_widget)
            .widget((1, 1), &mem_widget)
            .build(),
        WorkspaceBuilder::default()
            .name("Workspace 1")
            .widget((0, 0), &inst_widget)
            .widget((0, 1), &gpout_widget)
            .widget((1, 0), &reg_widget)
            .widget((1, 1), &mem_widget)
            .build(),
        WorkspaceBuilder::default()
            .name("Workspace 2")
            .widget((0, 0), &inst_widget)
            .widget((0, 1), &display_widget)
            .widget((1, 0), &reg_widget)
            .widget((1, 1), &mem_widget)
            .build(),
    ];

    UI::new(emu, workspaces).run(&mut ratatui::init())?;
    ratatui::restore();

    Ok(())
}
