use std::collections::HashMap;

fn blink(n: Vec<String>) -> Vec<String> {
    n.iter()
        .map(|x| blink_str(x.to_string()))
        .flatten()
        .collect()
}

fn remove_leading_zero(n: String) -> String {
    let mut n = n;
    while n.starts_with("0") && n.len() > 1 {
        n = n[1..].to_string();
    }
    n
}

fn blink_str(n: String) -> Vec<String> {
    if n == "0" {
        return vec![String::from("1")];
    }

    let n_len = n.len();
    if n_len % 2 == 0 {
        return vec![
            remove_leading_zero(n.to_string()[..(n_len / 2)].to_string()),
            remove_leading_zero(n.to_string()[(n_len / 2)..].to_string()),
        ];
    }

    return vec![(n.parse::<u64>().unwrap() * 2024).to_string()];
}

pub fn main() {
    let input = std::fs::read_to_string("inputs/day11.input").unwrap();

    // Part 1
    let x: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    let y = (0..25).fold(x, |acc, _| blink(acc));
    println!("Part 1: {:?}", y.len());

    // Part 2
    let x: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    let c: HashMap<String, u64> = x.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x.to_string()).or_insert(0) += 1;
        acc
    });
    let ctr: HashMap<String, u64> = (0..75).fold(c, |acc, _| {
        acc.iter()
            .map(|(k, v)| (k.to_string(), *v))
            .fold(HashMap::new(), |mut acc, (k, v)| {
                let y = blink_str(k.to_string());
                for x in y {
                    *acc.entry(x.to_string()).or_insert(0) += v;
                }
                acc
            })
    });
    println!("Part 2: {}", ctr.values().sum::<u64>());
}
