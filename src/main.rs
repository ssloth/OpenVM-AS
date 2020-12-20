extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::*;
use std::path::Path;
use std::process;

fn compile_line(line: &str) {
    let words = line.split_whitespace();
    for word in words {
        print!("{} ", word);
    }
}

fn compile_file(data: BufReader<File>) {
    for line in data.lines() {
        let line_str: String = line.unwrap();
        let str = line_str.split(';').collect::<Vec<&str>>();
        compile_line(str[0]);
        println!();
    }
}

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
            let f = File::open(file).expect("File not found");
            let fin = BufReader::new(f);
            compile_file(fin);
        } else {
            eprintln!("error No such file");
            process::exit(1);
        }
    }
}
