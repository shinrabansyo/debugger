use sb_emu::step;
use sb_emu::State as EmuState;
use sb_dbg_utils::setup_from_args;

fn main() -> anyhow::Result<()> {
    let (pc, dmem, imem) = setup_from_args()?;
    let mut emu = EmuState::new(pc, &dmem, &imem);
    loop {
        emu = step(emu)?;
        update_stdout(&emu);
    }
}

fn update_stdout(emu: &EmuState) {
    static mut OUTPUT_POS: usize = 0;

    let bef_output_pos = unsafe { OUTPUT_POS };
    let uart_output = emu.devices.get_stat(0).unwrap();
    if uart_output.len() > bef_output_pos {
        print!("{}", &uart_output[bef_output_pos..]);
        unsafe { OUTPUT_POS = uart_output.len() };
    }
}
