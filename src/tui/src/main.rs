use sb_emu::Emulator;
use sb_dbg_tui::prelude::*;
use sb_dbg_tui::widget::{Inst, Register, Mem, Gpout, Uart, Display};
use sb_dbg_tui::{UI, WorkspaceBuilder};
use sb_dbg_utils::setup_from_args;

fn main() -> anyhow::Result<()> {
    let (pc, dmem, imem) = setup_from_args()?;
    let emu = Emulator::new(pc, &dmem, &imem);

    let inst_widget = Inst::new();
    let reg_widget = Register::new();
    let mem_widget = Mem::new();
    let gpout_widget = Gpout::new();
    let uart_widget = Uart::new();
    let display_widget = Display::<128, 128>::new();

    let workspaces = [
        /* Main View
        +------------+---------------------+
        |            |            |  GPIO  |
        |            |   Memory   +--------|
        |            |            |        |
        |            +------------+  UART  |
        |            |  Register  |        |
        |            +------------+--------|
        |    Inst    |                     |
        |            |                     |
        |            |       Display       |
        |            |                     |
        |            |                     |
        +------------+---------------------+
        */
        WorkspaceBuilder::default()
            .name("Main View")
            .layout(|l| {
                l.split_h(100, |l| {
                    l.put(35, &inst_widget);
                    l.split_v(65, |l| {
                        l.split_h(50, |l| {
                            l.split_v(60, |l| {
                                l.put(60, &mem_widget);
                                l.put(40, &reg_widget);
                            });
                            l.split_v(40, |l| {
                                l.put(30, &gpout_widget);
                                l.put(70, &uart_widget);
                            });
                        });
                        l.put(50, &display_widget);
                    });
                });
            })
            .build(),
    ];

    UI::start(emu, workspaces)
}
