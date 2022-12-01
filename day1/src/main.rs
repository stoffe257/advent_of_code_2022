use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()>{

    let file = File::open("input.in")?;
    let reader = BufReader::new(file);

    let mut top_three: Vec<i32> = vec![0,0,0];
    let mut count: i32 = 0;

    for row in reader.lines() {
        let val = row?;

        if val == ""{
            if count > top_three[0]{
                top_three[0] = count;
                top_three.sort();
            }
            count = 0;

        } else {
            count += val.parse::<i32>().unwrap();
        }

    }

    if count > top_three[0]{
        top_three[0] = count;
    }

    let tot_sum: i32 = top_three.iter().sum();

    println!("Sum of top three: {}", tot_sum);

    Ok(())
}


fn _main_1() -> io::Result<()>{

    let file = File::open("input.in")?;
    let reader = BufReader::new(file);

    let mut max_count: i32 = 0;
    let mut count: i32 = 0;

    for row in reader.lines() {
        let val = row?;

        if val == ""{
            if count > max_count{
                max_count = count;
            }
            count = 0;

        } else {
            count += val.parse::<i32>().unwrap();
        }

    }
    if count > max_count{
        max_count = count;
    }

    println!("Max count: {}", max_count);


    Ok(())
}
