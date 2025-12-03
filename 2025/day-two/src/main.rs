use std::fs;

fn main() {
    let mut invalid_ids: u128 = 0;
    let txt = fs::read_to_string("file.txt").unwrap();

    for i in txt.split(",") {
        let i = i.trim();
        if i.is_empty() {
            continue;
        }

        let (start, finish) = i.split_once("-").expect("string must contain -");
        let start: u128 = start.parse().unwrap();
        let finish: u128 = finish.parse().unwrap();

        // FIXED: include the end number
        for n in start..=finish {
            if invalid_id(n) {
                invalid_ids += n;
            }
        }
    }

    println!("{}", invalid_ids);
}

fn invalid_id(n: u128) -> bool {
    let s = n.to_string();
    repeated_substring_pattern(&s)
}

fn repeated_substring_pattern(s: &str) -> bool {
    let doubled = format!("{}{}", s, s);
    doubled[1..doubled.len() - 1].contains(s)
}

