use std::{collections::btree_map::OccupiedEntry, fs};

fn main() {
    let data = fs::read_to_string("file.txt").unwrap();

    //first part

    let mut rows: Vec<Vec<&str>> = Vec::new();
    for raw in data.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let line: Vec<&str> = line.split(' ').filter(|&x| !x.is_empty()).collect();
        rows.push(line);
    }

    print!("{:?}", rows);
    let grand_total: u128 = solve_engine_fixed(rows);
    println!("grand total is: {}", grand_total);
}

// first problem
fn _solve_engine(data: Vec<Vec<&str>>) -> u128 {
    let m = data.len();
    let n = data[0].len();
    let mut total: u128 = 0;
    for j in 0..n {
        let operation = data[m - 1][j];
        let mut temp_total = match operation {
            "+" => 0u128,
            "*" => 1u128,
            _ => panic!("invalid operation"),
        };
        for i in 0..m - 1 {
            let val = data[i][j].parse::<u128>().expect("number is expected");
            match operation {
                "+" => temp_total += val,
                "*" => temp_total *= val,
                _ => unreachable!(),
            }
        }
        total += temp_total;
    }
    total
}

// second problem

fn solve_engine_fixed(data: Vec<Vec<&str>>) -> u128 {
    let m = data.len();
    let n = data[0].len();

    let mut total: u128 = 0;
    for j in 0..n {
        let operation = data[m - 1][j];
        let mut temp_total = match operation {
            "+" => 0u128,
            "*" => 1u128,
            _ => panic!("invalid operation"),
        };
        for i in 0..m - 1 {
            let split_num: Vec<char> = data[i][j].chars().collect();
            
        }
    }
    0
}
