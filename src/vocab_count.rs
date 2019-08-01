use std::collections::HashMap;

use std::hash::BuildHasherDefault;
use std::io::Write as IoWrite;

use fasthash;
use fasthash::xx::Hasher64;

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .about("Tool to extract unigram counts")
        .arg(clap::Arg::with_name("verbose")
            .long("verbose")
            .takes_value(true)
            .default_value("2")
            .possible_values(&["0", "1", "2"])
            .help("Sets the level of verbosity"))
        .arg(clap::Arg::with_name("min-count")
            .long("min-count")
            .takes_value(true)
            .help("Lower limit such that words which occur fewer than <min-count> times are discarded"))
        .arg(clap::Arg::with_name("max-vocab")
            .long("max-vocab")
            .takes_value(true)
            .help("Upper bound on vocabulary size, i.e. keep the <int> most frequent words. The minimum frequency words are randomly sampled so as to obtain an even distribution over the alphabet"))
        .arg(clap::Arg::with_name("input-file"))
        .arg(clap::Arg::with_name("output-file"))
        .get_matches();
    let input_reader = std::io::BufReader::new(
        std::fs::File::open(matches.value_of("input-file").unwrap())
            .expect("Can't open source file"),
    );
    let counts = get_counts(input_reader);
    let mut output_writer = std::io::BufWriter::new(
        std::fs::File::create(matches.value_of("output-file").unwrap())
            .expect("Can't open target file"),
    );
    for (word, count) in counts.iter(){
        writeln!(&mut output_writer, "{} {}", word, count.to_string()).expect("Couldn't write to output");
    }
}


fn get_counts<R: std::io::BufRead>(input: R) -> HashMap<String, u64, BuildHasherDefault<Hasher64>> {
    let mut counts = HashMap::<String, u64, BuildHasherDefault<Hasher64>>::default();
    for line in input.lines(){
        for word in line.unwrap().split_whitespace(){
            *counts.entry(word.to_string()).or_insert(0) += 1;
        }
    }
    counts
}