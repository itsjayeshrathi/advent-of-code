use std::fs;

fn main() {
    let mut fresh_item: u128 = 0;
    let mut ids_to_check: Vec<u128> = Vec::default();
    let mut ranges: Vec<(u128, u128)> = Vec::default();
    let data = fs::read_to_string("file.txt").unwrap();

    for raw in data.lines() {
        let line = raw.trim();
        if line.contains("-") {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<u128>().unwrap();
            let end = parts[1].parse::<u128>().unwrap();
            ranges.push((start, end));
        } else if !line.is_empty() {
            ids_to_check.push(line.parse::<u128>().unwrap());
        }
    }

    //first part
    // for id in ids_to_check {
    //     for &(start, end) in &ranges {
    //         if id >= start && id <= end {
    //             fresh_item += 1;
    //             break;
    //         }
    //     }
    // }

    //second part
    let res = merge(ranges);
    for &(start, end) in &res {
        fresh_item += (end - start) + 1;
    }

    println!("{}", fresh_item);
}

fn merge(mut ranges: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    ranges.sort_by_key(|r| r.0);

    let mut merged = Vec::new();
    let mut prev = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= prev.1 {
            prev.1 = prev.1.max(end);
        } else {
            merged.push(prev);
            prev = (start, end);
        }
    }

    merged.push(prev);

    merged
}
