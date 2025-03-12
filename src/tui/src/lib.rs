use sb_emu::Emulator;

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
    let emu = Emulator::new(pc, &dmem, &imem);

    let inst_widget = Inst::new();
    let reg_widget = Register::new();
    let mem_widget = Mem::new();
    let gpout_widget = Gpout::new();
    let uart_widget = Uart::new();
    let display_widget = Display::<128, 128>::new();

    let workspaces = [
        WorkspaceBuilder::default()
            .name("Workspace 0")
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
                                l.put(50, &gpout_widget);
                                l.put(50, &uart_widget);
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
