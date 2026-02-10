use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use sb_asm::assemble;

pub fn load_assembly(path: &Path) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let asm = read_file(path)?;
    let (dmem, imem) = assemble(&asm)?;
    let (dmem, imem) = (str_to_bytechar(&dmem), str_to_bytechar(&imem));
    Ok((dmem, imem))
}

pub fn load_hexfile(path: &Path) -> anyhow::Result<Vec<u8>> {
    let bytechar = read_file(path)?;
    let bytechar = str_to_bytechar(&bytechar);
    Ok(bytechar)
}

pub fn load_bytechar(
    d_path: Option<std::path::PathBuf>,
    i_path: &Path,
) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let imem = load_hexfile(i_path)?;
    let dmem = match d_path {
        Some(path) => load_hexfile(&path)?,
        None => vec![],
    };
    Ok((dmem, imem))
}

fn read_file(path: &Path) -> anyhow::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}

// 任意 byte の16進文字列を Vec<u8> に変換
fn hexstr_to_le_bytes(hex_str: &str) -> anyhow::Result<Vec<u8>> {
    if hex_str.len() % 2 != 0 {
        return Err(anyhow::anyhow!("Hex string has odd length"));
    }
    let mut bytes = (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>()?;
    bytes.reverse();

    Ok(bytes)
}

// 改行区切りの16進文字列を Vec<u8> に変換
fn str_to_bytechar(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for line in s.lines() {
        let bytes = hexstr_to_le_bytes(line).unwrap();
        result.extend_from_slice(&bytes);
    }
    result
}
