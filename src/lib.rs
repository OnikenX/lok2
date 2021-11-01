use std::{io};

use to_binary::BinaryString;

//[compressed][0/1]                      [0] [0  / 1]; [1][0 / 1]
// const LOK_TYPES: [[&'static str; 2]; 2] = [["lol", "ok"], ["l", "k"]];
pub enum LokType {
    Compressed,
    Uncompressed,
}
impl LokType {
    pub fn get_ok(&self) -> &'static str {
        match self {
            LokType::Compressed => "k",
            LokType::Uncompressed => "ok",
        }
    }

    pub fn get_lol(&self) -> &'static str {
        match self {
            LokType::Compressed => "l",
            LokType::Uncompressed => "lol",
        }
    }
}

pub fn from_string_to_lok(buffer: String, lok_type: LokType) -> String {
    //transforming to binary
    let bin = BinaryString::from(buffer);
    let mut bin_str = bin.to_string();

    //giving spaces
    match lok_type {
        LokType::Uncompressed => {
            bin_str = bin_str.replace("0", "0 ").replace("1", "1 ");
        }
        _ => {}
    }
    //transform to lok
    bin_str = bin_str
        .replace("0", lok_type.get_lol())
        .replace("1", lok_type.get_ok());
    return bin_str;
}

pub fn from_lok_to_string(buffer: String, lok_type: LokType) -> io::Result<String> {
    //removes spaces
    let mut buffer = buffer;
    buffer.retain(|c| !c.is_whitespace());
    buffer = buffer
        .replace(lok_type.get_lol(), "0")
        .replace(lok_type.get_ok(), "1");

    let x = from_binary_string_to_utf8_string(buffer.to_string());
    return x;
}

fn from_binary_string_to_utf8_string(bin_str: String) -> io::Result<String> {
    let mut bytes = vec![];
    let mut bin_str = bin_str;
    while bin_str.len() >= 8 {
        let (first, second) = bin_str.split_at(8);
        let byte;
        match from_string_8bits_to_byte(first) {
            Ok(b) => byte = b,
            Err(e) => {
                eprintln!("Error converting lok to string: {}", e);
                panic!("{}", e);
            }
        }
        bytes.push(byte);
        bin_str = second.to_owned();
    }
    let x = String::from_utf8(bytes);
    match x {
        Ok(x) => Ok(x),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Error converting output: {}", e)))
    }
}

fn from_string_8bits_to_byte(input: &str) -> Result<u8, std::io::Error> {
    if input.len() != 8 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "The string needs to be 8 bytes.",
        ));
    }
    if !input.is_ascii() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "The string needs to be in ascii only, with only 1s and 0s.",
        ));
    }

    let mut result: u8 = 0;
    for (index, char) in input.as_bytes().iter().enumerate() {
        match char {
            49 => {
                result |= 2u8.pow(7 - index as u32);
            }
            48 => {}
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!(
                        "The string can only have 1s or 0s: at {} has invalid character {}",
                        index, *char as char
                    ),
                ));
            }
        };
    }
    Ok(result)
}
