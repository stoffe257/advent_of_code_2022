use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.in")?;
    let reader = BufReader::new(file);

    let mut count: u32 = 0;

    for row in reader.lines() {
        let bag: Vec<u32> = row?.chars().map(|v| v as u32).collect::<Vec<_>>();
        let size = bag.len();

        let dst: Vec<Vec<u32>> = bag.chunks(size / 2).map(|s| s.into()).collect();

        let comp1 = &dst[0];
        let comp2 = &dst[1];

        let mut set: BTreeSet<u32> = BTreeSet::new();

        for i in comp1 {
            set.insert(*i);
        }

        for i in comp2 {
            if set.contains(i) {
                if *i > 96 && *i < 123 {
                    count += *i - 96;
                } else {
                    count += *i - 38;
                }
                break;
            }
        }
    }

    println!("Count: {}", count);

    Ok(())
}
