use std::fs::{read_to_string, File};
use std::io::{self, prelude::*, BufReader};

fn main() {
    let file_name: &str = "input.in";
    let _a = part1(file_name);
    // let _b = part2(file_name);
}

fn part1(file_name: &str) -> io::Result<()> {
    let file = File::open(file_name)?;

    let lines: Vec<_> = BufReader::new(file).lines().map(|s| s.unwrap()).collect();

    let chars: Vec<char> = lines[0].chars().collect();

    let mut last_four: Vec<char> = vec![chars[0], chars[1], chars[2], chars[3]];

    let mut count: usize = 0;
    for c in chars {
        last_four[count % 4] = c;
        count += 1;

        if !(1..last_four.len()).any(|i| last_four[i..].contains(&last_four[i - 1])) {
            break;
        }
    }

    println!("{} characters need to be read!", count);

    Ok(())
}

fn part2(file_name: &str) -> io::Result<()> {
    let input = include_str!("../input.in");

    Ok(())
}
