use std::{io, panic, process::{self, exit}};
use structopt::StructOpt;

use lok2::{self, LokType, from_lok_to_string, from_string_to_lok};

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
    get_input();

    let result;
    if opt.encode {
        result = from_string_to_lok(get_input(), lok_type);
    } else {
        match from_lok_to_string(get_input(), lok_type){
            Ok(r) => result = r,
            Err(e) => {
                eprint!("{}",e);
                exit(1);
            }
        }
    }
    
    print!("{}", result);
}
