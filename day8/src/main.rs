use std::io;

fn main() {
    let _a = part1();
    let _b = part2();
}

fn part2() -> io::Result<()> {
    let input = include_str!("../input.in");

    let mut forrest_matrix: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();

    for row in input.lines() {
        let tmp: Vec<u8> = row.chars().map(|c| c as u8 - 48).collect();
        forrest_matrix.append(&mut vec![tmp]);
    }

    let mut best: u32 = 0;
    for row in 1..forrest_matrix.len() - 1 {
        for col in 1..forrest_matrix[0].len() - 1 {
            let res = scenic_score(row, col, &forrest_matrix);
            if res > best {
                best = res;
            }
        }
    }

    println!("Best scenic score is: {}", best);

    Ok(())
}

fn scenic_score(row: usize, col: usize, forrest_matrix: &Vec<Vec<u8>>) -> u32 {
    let mut score: Vec<u32> = vec![0, 0, 0, 0];
    let curr_tree_hight: u8 = forrest_matrix[row][col];

    // up
    for r in (0..row).rev() {
        if forrest_matrix[r][col] >= curr_tree_hight {
            score[0] += 1;
            break;
        }
        score[0] += 1;
    }

    // left
    for c in (0..col).rev() {
        if forrest_matrix[row][c] >= curr_tree_hight {
            score[1] += 1;
            break;
        }
        score[1] += 1;
    }

    // right
    for c in col + 1..forrest_matrix[0].len() {
        if forrest_matrix[row][c] >= curr_tree_hight {
            score[2] += 1;
            break;
        }
        score[2] += 1;
    }

    // down
    for r in row + 1..forrest_matrix.len() {
        if forrest_matrix[r][col] >= curr_tree_hight {
            score[3] += 1;
            break;
        }
        score[3] += 1;
    }

    score[0] * score[1] * score[2] * score[3]
}

fn part1() -> io::Result<()> {
    let input = include_str!("../input.in");

    let mut forrest_matrix: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();

    for row in input.lines() {
        let tmp: Vec<u8> = row.chars().map(|c| c as u8 - 48).collect();
        forrest_matrix.append(&mut vec![tmp]);
    }

    let mut visible: Vec<Vec<bool>> =
        vec![vec![false; forrest_matrix[0].len()]; forrest_matrix.len()];

    for row in 0..forrest_matrix.len() {
        count_visable_trees_in_row(&mut forrest_matrix[row], &mut visible[row]);
    }

    for col in 0..forrest_matrix[0].len() {
        let mut tree_column: Vec<u8> = forrest_matrix.iter().map(|x| x[col]).collect();
        let mut visible_column: Vec<bool> = visible.iter().map(|x| x[col]).collect();
        count_visable_trees_in_row(&mut tree_column, &mut visible_column);

        for j in 0..forrest_matrix.len() {
            visible[j][col] = visible_column[j];
        }
    }

    let mut count2: u32 = 0;
    for i in visible {
        count2 += i.into_iter().filter(|b| *b).collect::<Vec<bool>>().len() as u32;
    }

    println!("Amount of visible trees: {}", count2);

    Ok(())
}

fn count_visable_trees_in_row(tree_row: &mut Vec<u8>, visable_row: &mut Vec<bool>) {
    let mut visible: Vec<bool> = Vec::<bool>::new();
    visible.resize(tree_row.len(), false);

    let mut highest: i8 = -1;

    for i in 0..tree_row.len() {
        if tree_row[i] as i8 > highest {
            highest = tree_row[i] as i8;
            visable_row[i] = true;
        }
    }
    highest = -1;
    for i in (0..tree_row.len()).rev() {
        if tree_row[i] as i8 > highest {
            highest = tree_row[i] as i8;
            visable_row[i] = true;
        }
    }
}
