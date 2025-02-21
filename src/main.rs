mod opts;
mod load;

use opts::{parse_args, FileFormat};
use load::{load_assembly, load_bytechar, load_bytechar2};

fn main() -> anyhow::Result<()> {
    let options = parse_args();
    let (dmem, imem) = match (options.format, options.data) {
        (FileFormat::Assembly, _) => load_assembly(&options.inst)?,
        (FileFormat::ByteChar, Some(data)) => load_bytechar2(&data, &options.inst)?,
        (FileFormat::ByteChar, None) => (load_bytechar(&options.inst)?, vec![]),
    };

    sb_dbg_tui::run(options.pc, &dmem, &imem)
}
