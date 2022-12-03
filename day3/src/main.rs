use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.in")?;
    let lines: Vec<_> = BufReader::new(file).lines().map(|s| s.unwrap()).collect();

    let mut count: u32 = 0;

    for i in 0..lines.len() / 3 {
        let bag1: Vec<_> = lines[3 * i].chars().collect();
        let bag2: Vec<_> = lines[3 * i + 1].chars().collect();
        let bag3: Vec<_> = lines[3 * i + 2].chars().collect();

        let mut set1: BTreeSet<char> = BTreeSet::new();
        let mut set3: BTreeSet<char> = BTreeSet::new();

        for i in bag1 {
            set1.insert(i);
        }

        for i in bag3 {
            set3.insert(i);
        }

        for i in bag2 {
            if set1.contains(&i) && set3.contains(&i) {
                let value: u32 = i as u32;
                if value > 96 && value < 123 {
                    count += value - 96;
                } else {
                    count += value - 38;
                }
                break;
            }
        }
    }

    println!("Count: {}", count);

    Ok(())
}

fn __main() -> io::Result<()> {
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
