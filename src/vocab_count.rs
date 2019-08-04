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
        .about("Tool to extract unigram counts")
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
    let mut counts = get_counts(input_reader);
    let mut output_writer = std::io::BufWriter::new(
        std::fs::File::create(matches.value_of("output-file").unwrap())
            .expect("Can't open target file"),
    );

    counts = filter_counts(
        counts,
        matches.value_of("min-count").map(|s| s.parse().unwrap()),
        matches.value_of("max-vocab").map(|s| s.parse().unwrap()),
    );
    for (word, count) in counts.iter() {
        writeln!(&mut output_writer, "{} {}", word, count.to_string())
            .expect("Couldn't write to output");
    }
}

fn get_counts<R: std::io::BufRead>(input: R) -> HashMap<String, u64, BuildHasherDefault<Hasher64>> {
    let mut counts = HashMap::<String, u64, BuildHasherDefault<Hasher64>>::default();
    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_draw_target(indicatif::ProgressDrawTarget::stderr());
    let mut n_words = 0;
    for line in input.lines() {
        for word in line.unwrap().split_whitespace() {
            *counts.entry(word.to_string()).or_insert(0) += 1;
            n_words += 1
        }
        pb.set_message(&format!("Read {} words", n_words));
    }
    counts
}

fn filter_counts<
    U: std::cmp::Eq + std::hash::Hash,
    T: std::hash::BuildHasher + std::default::Default,
>(
    mut counts: HashMap<U, u64, T>,
    min_count: Option<u64>,
    max_vocab: Option<usize>,
) -> HashMap<U, u64, T> {
    match min_count {
        Some(n) => counts.retain(|_, &mut v| v >= n),
        _ => (),
    }
    match max_vocab {
        Some(n) if n < counts.len() => {
            let mut counts_vec: Vec<(U, u64)> = counts.into_iter().collect();
            counts_vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));
            counts = counts_vec.drain(..n).collect();
        }
        _ => (),
    }
    return counts;
}
