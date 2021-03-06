use std::fs::File;
use std::io::BufReader;

use nb2nl::{xml2nl, nl2xml};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} [input]", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    if input.ends_with(".xml") {
        let xml = BufReader::new(File::open(input).expect("failed to open file"));
        let netlogo = xml2nl::parse(xml).expect("failed to translate");
        println!("{}", netlogo);
    }
    else if input.ends_with(".nlogo") {
        let content = std::fs::read_to_string(input).expect("failed to open file");
        let prog_stop = content.find("@#$#@#$#@").unwrap_or_else(|| content.len());
        let program = &content[..prog_stop];

        match nl2xml::parse(&input[..input.len()-6], program) {
            Ok(xml) => println!("{}", xml),
            Err(e) => eprintln!("{}", e),
        }
    }
    else {
        eprintln!("unknown input file type");
        std::process::exit(1);
    }
}
