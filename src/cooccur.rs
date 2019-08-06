use std::collections::HashMap;

use std::hash::BuildHasherDefault;
use std::io::Write as IoWrite;

use fasthash;
use fasthash::xx::Hasher64;

use indicatif;

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .about("Tool to calculate word-word cooccurrence statistics")
        .arg(clap::Arg::with_name("window-size")
            .long("window-size")
            .takes_value(true)
            .default_value("15")
            .help("Number of context words to the left (and to the right, if --symetric is specified)"))
        .arg(clap::Arg::with_name("input-file"))
        .arg(clap::Arg::with_name("output-file"))
        .get_matches();
    let input_reader = std::io::BufReader::new(
        std::fs::File::open(matches.value_of("input-file").unwrap())
            .expect("Can't open source file"),
    );
    let mut output_writer = std::io::BufWriter::new(
        std::fs::File::create(matches.value_of("output-file").unwrap())
            .expect("Can't open target file"),
    );
}