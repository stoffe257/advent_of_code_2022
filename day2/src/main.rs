use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()>{

    let file = File::open("input.in")?;
    let reader = BufReader::new(file);

    let mut score: u32 = 0;

    for row in reader.lines() {
        let turn = row?;

        let elf_hand = turn.chars().nth(0).unwrap() as u32 - 64;
        let result = turn.chars().nth(2).unwrap() as u32 - 87;

        score += match result{
            2 => 3,
            3 => 6,
            1 => 0,
            _ => todo!()
        };

        if result == 2{
            // println!("Draw");
            score += elf_hand;
        } else if result == 1{
            // println!("Loss");
            score += match elf_hand{
                1 => 3,
                2 => 1,
                _ => 2,
            }
        } else {
            // println!("Win");
            score += match elf_hand{
                1 => 2,
                2 => 3,
                _ => 1,
            }
        }

    }

    println!("Score: {}", score);

    Ok(())
}


fn __main() -> io::Result<()>{

    let file = File::open("input.in")?;
    let reader = BufReader::new(file);

    let mut score: u32 = 0;

    for row in reader.lines() {
        let turn = row?;

        let elf_hand = turn.chars().nth(0).unwrap() as u32 - 64;
        let own_hand = turn.chars().nth(2).unwrap() as u32 - 87;

        score += own_hand;

        if own_hand == elf_hand{
            score += 3;
        } else if own_hand == 1 && elf_hand == 3{
            score += 6;
        } else if own_hand == 2 && elf_hand == 1{
            score += 6;
        } else if own_hand == 3 && elf_hand == 2{
            score += 6;
        } else{
            continue;
        }

    }

    println!("Score: {}", score);

    Ok(())
}
