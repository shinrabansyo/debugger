use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

pub fn load_bytechar(path: &Path) -> anyhow::Result<Vec<u8>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result = Vec::new();
    for line in reader.lines() {
        let byte = u8::from_str_radix(&line?, 16)?;
        result.push(byte);
    }

    Ok(result)
}
