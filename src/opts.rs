use std::path::PathBuf;
use std::str::FromStr;

use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct Options {
    /// Initial PC value [default: 0]
    #[bpaf(short, long, fallback(0))]
    pub pc: u32,
    /// D-Mem file (optional)
    #[bpaf(short, long)]
    pub data: Option<PathBuf>,
    /// I-Mem file
    #[bpaf(short, long)]
    pub inst: PathBuf,
    /// File format [default: bytechar]
    #[bpaf(short, long, fallback(FileFormat::ByteChar))]
    pub format: FileFormat,
}

#[derive(Debug, Clone)]
pub enum FileFormat {
    ByteChar,
}

impl FromStr for FileFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "bytechar" => Ok(FileFormat::ByteChar),
            _ => Err(anyhow::anyhow!("Invalid file format")),
        }
    }
}

pub fn parse_args() -> Options {
    options().run()
}
