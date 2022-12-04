use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/* A faster way would be to have the first interval in a vector
    and then to add the lower bound of interval 2 and sort the vector.
    Repeat for the higher bound of interval 2 if needed. 
    Looking at which position the entered bounds end up
    should solve both parts faster */

fn main(){
    let file_name: &str = "input.in";
    let _a = part1(file_name);
    let _b = part2(file_name);
}

fn part2(file_name: &str) -> io::Result<()>{
    let file = File::open(file_name)?;
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

        let mut interval1: BTreeSet<u32> = BTreeSet::new();
        for i in areas[0]..areas[1]+1{
            interval1.insert(i);
        }

        for i in areas[2]..areas[3]+1{
            if interval1.contains(&i){
                count += 1;
                break;
            }
        }
    }

    println!("Count: {}", count);
    Ok(())
}

fn part1(file_name: &str) -> io::Result<()> {
    let file = File::open(file_name)?;
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
