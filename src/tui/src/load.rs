use std::fs::File;
use std::path::Path;
use std::io::{BufReader, Read};

use sb_asm::assemble;


pub fn load_assembly(path: &Path) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let asm = read_file(path)?;
    let (dmem, imem) = assemble(&asm)?;
    let (dmem, imem) = (str_to_bytechar(&dmem), str_to_bytechar(&imem));
    Ok((dmem, imem))
}

pub fn load_bytechar(path: &Path) -> anyhow::Result<Vec<u8>> {
    let bytechar = read_file(path)?;
    let bytechar = str_to_bytechar(&bytechar);
    Ok(bytechar)
}

pub fn load_bytechar2(d_path: &Path, i_path: &Path) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let dmem = load_bytechar(d_path)?;
    let imem = load_bytechar(i_path)?;
    Ok((dmem, imem))
}

fn read_file(path: &Path) -> anyhow::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}

fn str_to_bytechar(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for line in s.lines() {
        let byte = u8::from_str_radix(&line, 16).unwrap();
        result.push(byte);
    }
    result
}
