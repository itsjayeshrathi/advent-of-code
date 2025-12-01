use std::fs;

fn main() {
    let mut position = 50;
    let mut password = 0;

    let txt = fs::read_to_string("file.txt").unwrap();

    for raw in txt.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let dir = line.chars().next().unwrap();
        let rotation: i32 = line[1..].trim().parse().unwrap();

        let step: i32 = match dir {
            'L' => -1,
            'R' => 1,
            _ => panic!("Invalid direction"),
        };

        for _ in 0..rotation {
            position = (position + step).rem_euclid(100);

            if position == 0 {
                password += 1;
            }
        }
    }

    println!("Password: {}", password);
}
