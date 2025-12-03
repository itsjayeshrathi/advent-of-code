use std::fs;

fn main() {
    let mut sum = 0;
    let txt = fs::read_to_string("file.txt").unwrap();

    for raw in txt.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let chars: Vec<char> = line.chars().collect();
        //println!("{:?}", chars);
        let digit: u128 = get_code_two(chars);
        //println!("{}", digit);
        sum += digit;
    }
    println!("sum is {}", sum);
}

// first challenge
fn _get_code_one(chars: Vec<char>) -> u128 {
    let n = chars.len();
    let mut best = 0;

    for i in 0..n {
        let mut right_max = '\0';
        for j in (i + 1)..n {
            if chars[j] > right_max {
                right_max = chars[j];
            }
        }

        if right_max != '\0' {
            let candidate = format!("{}{}", chars[i], right_max).parse::<u32>().unwrap();
            if candidate > best {
                best = candidate;
            }
        }
    }

    best.into()
}

// second challenge
fn get_code_two(chars: Vec<char>) -> u128 {
    let k = 12;
    let n = chars.len();

    let mut stack: Vec<char> = Vec::new();
    let mut to_remove = n - k; 

    for &c in &chars {
        while let Some(&last) = stack.last() {
            if to_remove > 0 && last < c {
                stack.pop();
                to_remove -= 1;
            } else {
                break;
            }
        }
        stack.push(c);
    }

    // truncate to exactly k
    stack.truncate(k);

    // convert to number
    let s: String = stack.into_iter().collect();
    s.parse::<u128>().unwrap()
}
