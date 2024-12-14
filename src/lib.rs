use std::io::BufRead;
use std::io::Error;

pub struct CountData {
    pub bytes: usize,
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

pub fn get_count_data(mut reader: Box<dyn BufRead>) -> Result<CountData, Error> {
    let mut data = CountData {
        bytes: 0,
        lines: 0,
        words: 0,
        chars: 0,
    };

    loop {
        let mut line = String::new();
        let line_len = reader.read_line(&mut line)?;
        if line_len == 0 {
            break;
        }

        data.bytes += line.len();
        data.lines += 1;
        data.words += line.split_whitespace().count();
        data.chars += line.chars().count();
    }

    Ok(data)
}
