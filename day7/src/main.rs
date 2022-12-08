use std::collections::HashMap;
use std::io::{self};

fn main() {
    let file_name: &str = "test1.in";
    let _a = part1(file_name);
    // let _b = part2(file_name);
}

fn part1(_file_name: &str) -> io::Result<()> {
    let input = include_str!("../input.in");

    let mut dir_count: u64 = 0;
    let mut curr_dirs: Vec<String> = Vec::<String>::new();
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();

    for line in input.lines() {
        println!("Line -- {}", line);
        if line.chars().next().unwrap() == '$' {
            if line.contains("ls") {
                continue;
            } else if line.contains("cd ..") {
                curr_dirs.pop();
                println!("Stack: {:?}", curr_dirs);
            } else if line.contains("cd") {
                let name_str = &line[5..];
                let mut dir_name: String = name_str.to_string();
                dir_name.push_str(format!("_{}", dir_count).as_str());
                // let dir_name = name.as_str();
                curr_dirs.append(&mut vec![dir_name]);
                println!("Stack: {:?}", curr_dirs);
                dir_sizes.insert(dir_name, 0);
                // println!("Changed folder to {}", dir_name);
                dir_count += 1;
            }
        } else if line.starts_with("dir ") {
            continue;
            // println!("Dir: {}", &line[4..]);
        } else {
            let mut aa = line.split(" ");
            let file_tuple: (u64, &str) = (
                aa.next().unwrap().parse::<u64>().unwrap(),
                aa.next().unwrap(),
            );

            println!("File: {}, size: {}", file_tuple.1, file_tuple.0);
            print!("File added to hashmaps: ");
            for i in 0..curr_dirs.len() {
                print!("{}, ", curr_dirs[i]);
                let sizes = dir_sizes.get_mut(&curr_dirs[i]).unwrap();
                *sizes += file_tuple.0;
            }
            println!("");
        }
    }

    println!(" -------- Sum --------");
    let mut sum: u64 = 0;
    for dir in dir_sizes {
        println!("Dir {} has size: {}", dir.0, dir.1);
        if dir.1 < 100000 {
            sum += dir.1;
        }
    }

    println!("Sum: {}", sum);
    Ok(())
}
