use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.in")?;
    let reader = BufReader::new(file);

    let mut count: u32 = 0;

    for row in reader.lines() {
        let line = row?;
        let split: Vec<&str> = line.split(",").collect();

        let mut areas: Vec<u32> = Vec::<u32>::new();

        for interval in split{
            let values: Vec<&str> = interval.split("-").collect();
            for v in values{
                areas.append(&mut vec!(v.parse::<u32>().unwrap()));
            }
        }

        if (areas[0] <= areas[2] && areas[1] >= areas[3]) || 
            (areas[2] <= areas[0] && areas[3] >= areas[1]){
            count += 1;
        }
    }

    println!("Count: {}", count);
    Ok(())
}
