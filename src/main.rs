extern crate clap;

use clap::{App, Arg};

fn main() {
    let _matches = App::new("ovmas")
        .version("0.1.0")
        .author("LiuZY")
        .about("open vm's assembler")
        .arg(
            Arg::with_name("FILE")
                .help("File to print.")
                .empty_values(false),
        )
        .get_matches();
}
