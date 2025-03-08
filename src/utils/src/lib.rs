pub mod args;
pub mod load;

use args::{parse, FileFormat};
use load::{load_assembly, load_bytechar, load_bytechar2};

pub fn setup_from_args() -> anyhow::Result<(u32, Vec<u8>, Vec<u8>)> {
    let options = parse();
    let (dmem, imem) = match (options.format, options.data) {
        (FileFormat::Assembly, _) => load_assembly(&options.inst)?,
        (FileFormat::ByteChar, Some(data)) => load_bytechar2(&data, &options.inst)?,
        (FileFormat::ByteChar, None) => (load_bytechar(&options.inst)?, vec![]),
    };
    Ok((options.pc, dmem, imem))
}
