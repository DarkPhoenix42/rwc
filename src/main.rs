use clap::Parser;
use lib;
use std::io::BufRead;

#[derive(Parser, Debug)]
#[clap(
    author = "Praneeth Sarode",
    version = "1.0.0",
    about = "wc rewritten in rust."
)]
struct Args {
    #[arg(short = 'c')]
    /// Count the number of bytes.
    pub count_bytes: bool,

    #[arg(short = 'l')]
    /// Count the number of lines.
    pub count_lines: bool,

    #[arg(short = 'w')]
    /// Count the number of words.
    pub count_words: bool,

    #[arg(short = 'm')]
    /// Count the number of characters.
    pub count_chars: bool,

    #[arg(required = false, default_value = "")]
    /// The path to the file to analyse.
    pub file_path: String,
}

fn main() {
    let mut args = Args::parse();

    if !args.count_bytes && !args.count_lines && !args.count_words && !args.count_chars {
        args.count_bytes = true;
        args.count_lines = true;
        args.count_words = true;
        args.count_chars = true;
    }

    let reader: Box<dyn BufRead>;
    if args.file_path.is_empty() {
        reader = Box::new(std::io::BufReader::new(std::io::stdin()));
    } else {
        reader = Box::new(std::io::BufReader::new(
            std::fs::File::open(&args.file_path).unwrap(),
        ));
    }

    let data = lib::get_count_data(reader).unwrap();

    if args.count_bytes {
        print!("{}b ", data.bytes);
    }

    if args.count_lines {
        print!("{}l ", data.lines);
    }

    if args.count_words {
        print!("{}w ", data.words);
    }

    if args.count_chars {
        print!("{}c ", data.chars);
    }

    println!("{}", args.file_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn test_get_count_data() {
        let reader = Box::new(BufReader::new(File::open("test.txt").unwrap()));
        let data = lib::get_count_data(reader).unwrap();
        assert_eq!(data.bytes, 342190);
        assert_eq!(data.lines, 7145);
        assert_eq!(data.words, 58164);
        assert_eq!(data.chars, 339292);
    }
}
