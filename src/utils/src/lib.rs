pub mod args;
pub mod load;

use args::{parse, FileFormat};
use load::{load_assembly, load_bytechar};

pub fn setup_from_args() -> anyhow::Result<(u32, Vec<u8>, Vec<u8>, Option<std::path::PathBuf>)> {
    let options = parse();
    let (dmem, imem) = match options.format {
        FileFormat::Assembly => load_assembly(&options.inst)?,
        FileFormat::ByteChar => load_bytechar(options.data, &options.inst)?,
    };
    Ok((options.pc, dmem, imem, options.trace_path))
}
