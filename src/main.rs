use std::{io, panic, process};
use structopt::StructOpt;
use to_binary::BinaryString;

//[compressed][0/1]                      [0] [0  / 1]; [1][0 / 1]
// const LOK_TYPES: [[&'static str; 2]; 2] = [["lol", "ok"], ["l", "k"]];

enum LokType {
    Compressed,
    Uncompressed,
}
impl LokType {
    fn get_ok(&self) -> &'static str {
        match self {
            LokType::Compressed => "k",
            LokType::Uncompressed => "ok",
        }
    }

    fn get_lol(&self) -> &'static str {
        match self {
            LokType::Compressed => "l",
            LokType::Uncompressed => "lol",
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Lok",
    about = "An decoder/encoder of Lok/Ok, the language of the gods."
)]
struct Opt {
    ///Decodes an string passed by stdin
    #[structopt(short, long)]
    decode: bool,

    // ///Prints an help message
    // #[structopt(short, long)]
    // help: bool,
    ///Encodes an string passed by stdin
    #[structopt(short, long)]
    encode: bool,

    ///Uses the compressed method where the 'lol's and 'ok's are converted to 'l's and 'k's respectively
    #[structopt(short, long)]
    compressed: bool,
}

fn get_input() -> String {
    // Get input
    let mut buffer = String::new();
    eprintln!("Insert your message:");
    if let Err(e) = io::stdin().read_line(&mut buffer) {
        eprintln!("Error reading input: {}", e);
        panic!("{}", e);
    }

    buffer.trim().to_string()
}

fn from_string_to_lok(lok_type: LokType) {
    let buffer = get_input();

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
    println!("{}", bin_str);
}

fn from_lok_to_string(lok_type: LokType) {
    let mut buffer = get_input();

    //removes spaces
    buffer.retain(|c| !c.is_whitespace());

    buffer = buffer
        .replace(lok_type.get_lol(), "0")
        .replace(lok_type.get_ok(), "1");

    let x = from_binary_string_to_utf8_string(buffer.to_string());

    println!("{}", &x);
}

fn from_binary_string_to_utf8_string(bin_str: String) -> String {
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
    String::from_utf8(bytes).unwrap_or_else(|e| {
        eprintln!("Error converting output: {}",e);
        process::exit(1);
    })
}

fn main() {
    let opt = Opt::from_args();

    let lok_type;
    if (opt.decode && opt.encode) || (!opt.decode && !opt.encode) {
        eprintln!("You need to choose between encode or decode.");
        eprintln!("{}", Opt::clap().to_string());
        return;
    }
    if opt.compressed {
        lok_type = LokType::Compressed;
    } else {
        lok_type = LokType::Uncompressed;
    }

    if opt.encode {
        from_string_to_lok(lok_type);
    } else {
        from_lok_to_string(lok_type);
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
