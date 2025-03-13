pub use sb_dbg_tui_engine::widget::{Widget, WidgetView};
pub use sb_dbg_tui_engine::workspace::Workspace;
pub use sb_dbg_tui_engine::UI;

pub mod widget {
    pub use sb_dbg_tui_widget_device_display::Display;
    pub use sb_dbg_tui_widget_device_gpout::Gpout;
    pub use sb_dbg_tui_widget_device_uart::Uart;
    pub use sb_dbg_tui_widget_inst::Inst;
    pub use sb_dbg_tui_widget_reg::Register;
    pub use sb_dbg_tui_widget_mem::Mem;
}

pub mod prelude {
    pub use sb_dbg_tui_engine::widget::Widget;
}
