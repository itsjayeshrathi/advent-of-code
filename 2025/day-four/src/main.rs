use std::fs;

fn main() {
    let data = fs::read_to_string("file.txt").unwrap();
    let mut matrix: Vec<Vec<char>> = Vec::default();

    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let chars: Vec<char> = line.chars().collect();
        matrix.push(chars);
    }
    let res = traverse(matrix);

    println!("papers that can be removed: {}", res);
}

// first part
fn _traverse_one(matrix: Vec<Vec<char>>, mut papers_can_be_removed: u32) -> u32 {
    let n = matrix.len();
    let m = matrix[0].len();

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] != '@' {
                continue;
            }

            let mut adjacent = 0;

            for (dx, dy) in dirs {
                let ni = i as i32 + dx;
                let nj = j as i32 + dy;

                if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                    if matrix[ni as usize][nj as usize] == '@' {
                        adjacent += 1;
                    }
                }
            }

            if adjacent < 4 {
                papers_can_be_removed += 1;
            }
        }
    }

    println!("{:?}", matrix);
    papers_can_be_removed
}

fn traverse(mut matrix: Vec<Vec<char>>) -> u32 {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut total_removed = 0;

    loop {
        let mut removed_this_round = Vec::new();

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] != '@' {
                    continue;
                }

                let adj = count_adjacent(&matrix, i, j);

                if adj < 4 {
                    removed_this_round.push((i, j));
                }
            }
        }

        if removed_this_round.is_empty() {
            break;
        }

        for (i, j) in removed_this_round {
            matrix[i][j] = 'X';
            total_removed += 1;
        }
    }

    total_removed
}

fn count_adjacent(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let n = matrix.len() as i32;
    let m = matrix[0].len() as i32;
    let mut count = 0;

    for (dx, dy) in dirs {
        let ni = i as i32 + dx;
        let nj = j as i32 + dy;

        if ni >= 0 && ni < n && nj >= 0 && nj < m {
            if matrix[ni as usize][nj as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}
