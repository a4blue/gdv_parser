extern crate chardetng;
#[macro_use]
extern crate clap;
extern crate encoding;
extern crate gdv_parser_struct;
use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

arg_enum! {
    #[derive(Debug)]
    enum CharacterEncoding {
        Utf8,
        Guess
    }
}

fn main() {
    let matches = App::new("GDV-Parser")
        .version("0.1")
        .author("Alexander Ratajczak <a4blue@hotmail.de>")
        .about("Parses a GDV File")
        .arg(
            Arg::with_name("input")
                .value_name("INPUT_FILE")
                .default_value("test.gdv")
                .help("Path to the GDV File"),
        )
        .arg(
            Arg::with_name("output")
                .value_name("OUTPUT_FILE")
                .help("Not yet implemented. For now output ist stdOut"),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Not yet implemented. Output Format(XML etc.)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("character-encoding")
                .short("c")
                .long("character")
                .value_name("CHARACTER_ENCODING")
                .help("Only Utf8 implemented. Sets the character Encoding of the Input.")
                .possible_values(&CharacterEncoding::variants())
                .case_insensitive(true)
                .default_value("Utf8")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("timing")
                .short("t")
                .long("timing")
                .help("If set, Outputs the time it took to generate the GDV Structure"),
        )
        .get_matches();

    let path = Path::new(matches.value_of("input").unwrap());
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let character_encoding = value_t!(matches, "character-encoding", CharacterEncoding).unwrap();
    let gdv_string = match character_encoding {
        CharacterEncoding::Utf8 => {
            let mut string_buffer = String::new();
            if let Err(why) = file.read_to_string(&mut string_buffer) {
                panic!("Couldn't read {}: {}", display, why)
            };
            string_buffer
        }
        CharacterEncoding::Guess => String::new(),
    };

    let now = if matches.is_present("timing") {
        Option::from(Instant::now())
    } else {
        Option::None
    };

    gdv_parser_struct::parse(gdv_string);
    if let Some(time) = now {
        println!("Time:{} Seconds", time.elapsed().as_secs_f64())
    };
}
