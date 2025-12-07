use std::{collections::HashSet, fs};

fn main() {
    let data = fs::read_to_string("file.txt").unwrap();
    let mut tachyon_manifold: Vec<Vec<char>> = Vec::new();

    for raw in data.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let line: Vec<char> = line.chars().collect();
        tachyon_manifold.push(line);
    }
    println!("{:?}", tachyon_manifold);
    let split_count = get_tachyons_split(tachyon_manifold);
    println!("{}", split_count);
}

// first problem
fn get_tachyons_split(tachyon_manifold: Vec<Vec<char>>) -> u32 {
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

fn _get_quantum_split(tachyon_manifold: Vec<Vec<char>>) -> u32 {
    0
}
