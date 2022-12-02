use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()>{

    let file = File::open("input.in")?;
    let reader = BufReader::new(file);

    let mut score: u32 = 0;

    for row in reader.lines() {
        let turn = row?;

        let elf_hand = turn.chars().nth(0).unwrap() as u32 - 64;
        let own_hand = turn.chars().nth(2).unwrap() as u32 - 87;

        score += own_hand;

        if own_hand == elf_hand{
            // println!("Draw");
            score += 3;
        } else if own_hand == 1 && elf_hand == 3{
            // println!("Win");
            score += 6;
        } else if own_hand == 2 && elf_hand == 1{
            // println!("Win");
            score += 6;
        } else if own_hand == 3 && elf_hand == 2{
            score += 6;
        }else{
            // println!("Loss");
            continue;
        }

    }

    println!("Score: {}", score);

    Ok(())
}
