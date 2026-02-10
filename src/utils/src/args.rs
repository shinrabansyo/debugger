use std::path::PathBuf;
use std::str::FromStr;

use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct Options {
    /// I-Mem file
    #[bpaf(short, long)]
    pub inst: PathBuf,
    /// D-Mem file (optional)
    #[bpaf(short, long)]
    pub data: Option<PathBuf>,
    /// Initial PC value [default: 0]
    #[bpaf(short, long, fallback(0))]
    pub pc: u32,
    /// File format [default: assembly]
    #[bpaf(
        short,
        long,
        fallback(FileFormat::Assembly),
        argument("assembly|bytechar")
    )]
    pub format: FileFormat,
    /// Enable trace dump (optional)
    #[bpaf(short, long, argument("path to trace file"))]
    pub trace_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum FileFormat {
    Assembly,
    ByteChar,
}

impl FromStr for FileFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "assembly" => Ok(FileFormat::Assembly),
            "bytechar" => Ok(FileFormat::ByteChar),
            _ => Err(anyhow::anyhow!("Invalid file format")),
        }
    }
}

pub fn parse() -> Options {
    options().run()
}
