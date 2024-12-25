use itertools::iproduct;
use std::collections::HashMap;

fn mix(a: u64, m: u64) -> u64 {
    a ^ m
}

fn prune(a: u64) -> u64 {
    a % 16777216
}

fn secret(a: u64) -> u64 {
    let mut res = a;
    res = prune(mix(res * 64, res));
    res = prune(mix(res / 32, res));
    res = prune(mix(res * 2048, res));
    res
}

pub fn main() {
    let input = std::fs::read_to_string("inputs/day22.input").expect("Can't read file");
    let total: u64 = input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .map(|a| (0..2000).fold(a, |acc, _| secret(acc)))
        .sum();
    println!("Part 1: {:?}", total);

    // part 2
    let secrets: Vec<Vec<u64>> = input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .map(|a| {
            (0..2000).fold(vec![a], |mut acc, _| {
                acc.push(secret(acc[acc.len() - 1]));
                acc
            })
        })
        .collect();
    let changes: Vec<Vec<i64>> = secrets
        .iter()
        .map(|l| {
            let c = l.iter().map(|a| *a % 10).collect::<Vec<u64>>();
            c.windows(2)
                .map(|pair| pair[1] as i64 - pair[0] as i64)
                .collect::<Vec<i64>>()
        })
        .collect();
    let seqs: Vec<Vec<(Vec<i64>, u64)>> = changes
        .iter()
        .zip(secrets.iter())
        .map(|(c, s)| {
            c.windows(4)
                .zip(s[4..].iter())
                .map(|(w, a)| (w.to_vec(), a % 10))
                .collect::<Vec<(Vec<i64>, u64)>>()
        })
        .collect();

    // create map of sequences to value such that only the first occurrence of a sequence is kept
    let maps = seqs
        .iter()
        .map(|s| {
            let mut map: HashMap<Vec<i64>, u64> = HashMap::new();
            for (seq, val) in s.iter() {
                if map.contains_key(seq) {
                    continue;
                }
                *map.entry(seq.to_vec()).or_default() = *val;
            }
            map
        })
        .collect::<Vec<HashMap<Vec<i64>, u64>>>();

    let combinations: Vec<[i64; 4]> = iproduct!(-9..=9, -9..=9, -9..=9, -9..=9)
        .map(|(a, b, c, d)| [a, b, c, d])
        .collect();

    let res = combinations.iter().map(|comb| {
        let mut acc = 0;
        for map in maps.iter() {
            if let Some(val) = map.get(&comb.to_vec()) {
                acc += *val as i64;
            }
        }
        acc
    });
    println!("{:?}", res.max().unwrap());
}
