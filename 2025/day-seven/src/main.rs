use std::{collections::HashSet, fs};

fn main() {
    let data = fs::read_to_string("file.test.txt").unwrap();
    let mut tachyon_manifold: Vec<Vec<char>> = Vec::new();

    for raw in data.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let line: Vec<char> = line.chars().collect();
        tachyon_manifold.push(line);
    }
    let split_count: u64 = get_quantum_tachyons_split(tachyon_manifold);
    println!("{}", split_count);
}

// first problem
fn _get_tachyons_split(tachyon_manifold: Vec<Vec<char>>) -> u32 {
    let mut beam_tracker = HashSet::new();
    let mut split_count = 0;

    let n = tachyon_manifold[0].len();

    let add_if_valid = |set: &mut HashSet<usize>, j: isize| {
        if j >= 0 && (j as usize) < n {
            set.insert(j as usize);
        }
    };

    for row in tachyon_manifold {
        for (j, &cell) in row.iter().enumerate() {
            match cell {
                'S' => {
                    beam_tracker.insert(j);
                }
                '^' if beam_tracker.contains(&j) => {
                    // split beam safely
                    add_if_valid(&mut beam_tracker, j as isize - 1);
                    add_if_valid(&mut beam_tracker, j as isize + 1);
                    beam_tracker.remove(&j);
                    split_count += 1;
                }
                _ => {}
            }
        }
    }

    split_count
}

// second problem
fn get_quantum_tachyons_split(grid: Vec<Vec<char>>) -> u64 {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    // dp[r][c] = number of ways beam at (r,c) reaches bottom
    let mut dp = vec![vec![None; cols as usize]; rows as usize];

    fn dfs(
        r: isize,
        c: isize,
        g: &Vec<Vec<char>>,
        rows: isize,
        cols: isize,
        dp: &mut Vec<Vec<Option<u64>>>,
    ) -> u64 {
        // Out of bounds
        if c < 0 || c >= cols {
            return 0;
        }

        // Reached last row
        if r == rows - 1 {
            return 1;
        }

        // Memoized?
        if let Some(val) = dp[r as usize][c as usize] {
            return val;
        }

        let cell = g[r as usize][c as usize];

        let ways = match cell {
            '^' => {
                // split
                dfs(r + 1, c - 1, g, rows, cols, dp) + dfs(r + 1, c + 1, g, rows, cols, dp)
            }
            _ => {
                // straight
                dfs(r + 1, c, g, rows, cols, dp)
            }
        };

        dp[r as usize][c as usize] = Some(ways);
        ways
    }

    // Possible S positions on first row
    let mut total = 0;
    for (c, &ch) in grid[0].iter().enumerate() {
        if ch == 'S' {
            total += dfs(0, c as isize, &grid, rows, cols, &mut dp);
        }
    }

    total
}
