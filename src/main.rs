extern crate clap;

use clap::{App, Arg};
use std::path::Path;
use std::process;

fn main() {
    let matches = App::new("ovmas")
        .version("0.1.0")
        .author("LiuZY")
        .about("open-vm's assembler")
        .arg(
            Arg::with_name("FILE")
                .long("file")
                .short("f")
                .help("File to print.")
                .empty_values(false),
        )
        .get_matches();

    if let Some(file) = matches.value_of("FILE") {
        println!("value for file argument: {}", file);
        if Path::new(&file).exists() {
            
        } else {
            eprintln!("error No such file");
            process::exit(1);
        }
    }
}
