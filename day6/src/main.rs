use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let file_name: &str = "input.in";
    let _a = parts(file_name, 4);
    let _b = parts(file_name, 14);
}

fn parts(file_name: &str, read_len: usize) -> io::Result<()> {
    let file = File::open(file_name)?;
    let lines: Vec<_> = BufReader::new(file).lines().map(|s| s.unwrap()).collect();

    let chars: Vec<char> = lines[0].chars().collect();

    let mut last_nr: Vec<char> = Vec::<char>::from(&chars[0..read_len]);

    let mut count: usize = 0;
    for c in chars {
        last_nr[count % read_len] = c;
        count += 1;

        if !(1..last_nr.len()).any(|i| last_nr[i..].contains(&last_nr[i - 1])) {
            break;
        }
    }

    println!("{} characters need to be read!", count);

    Ok(())
}
