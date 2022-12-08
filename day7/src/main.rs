use std::collections::HashMap;
use std::io;

fn main() {
    let _a = part1();
    let _b = part2();
}

fn part2() -> io::Result<()> {
    let dir_sizes = calculate_dir_sizes();

    let total_space: u64 = 70000000;
    let needed_space: i64 = 30000000;

    let used_space: u64 = *dir_sizes.get("/").unwrap();

    let need_to_clear: u64 = -((total_space - used_space) as i64 - needed_space) as u64;

    let mut best_candidate_size: u64 = u64::MAX;
    for k in dir_sizes {
        if k.1 > need_to_clear && k.1 < best_candidate_size {
            best_candidate_size = k.1;
        }
    }

    println!("Size of best directory to delete: {}", best_candidate_size);

    Ok(())
}

fn part1() -> io::Result<()> {
    let dir_sizes = calculate_dir_sizes();

    let mut sum: u64 = 0;
    for dir in dir_sizes {
        if dir.1 < 100000 {
            sum += dir.1;
        }
    }

    println!("Sum: {}", sum);
    Ok(())
}

fn calculate_dir_sizes() -> HashMap<String, u64> {
    let input = include_str!("../input.in");

    let mut curr_dirs: Vec<String> = Vec::<String>::new();
    curr_dirs.append(&mut vec!["/".to_string()]);
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();
    dir_sizes.insert("/".to_string(), 0);

    for line in input.lines() {
        if line.chars().next().unwrap() == '$' {
            if line.starts_with("$ cd ..") {
                curr_dirs.pop();
            } else if line.starts_with("$ cd") && !line.starts_with("$ cd /") {
                let mut dir_name: String = curr_dirs.last().unwrap().to_string();
                dir_name.push_str("/");
                dir_name.push_str(&line[5..]);
                curr_dirs.append(&mut vec![dir_name.clone()]);
                dir_sizes.insert(dir_name, 0);
            }
        } else if !line.starts_with("dir ") {
            let mut aa = line.split(" ");
            let file_tuple: (u64, &str) = (
                aa.next().unwrap().parse::<u64>().unwrap(),
                aa.next().unwrap(),
            );

            for i in 0..curr_dirs.len() {
                let sizes = dir_sizes.get_mut(&curr_dirs[i]).unwrap();
                *sizes += file_tuple.0;
            }
        }
    }
    dir_sizes
}
