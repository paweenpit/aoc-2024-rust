use itertools::Itertools;

pub fn main() {
    let input = std::fs::read_to_string("inputs/day25.input").expect("error reading file");
    let keys_locks = input.split("\n\n").filter_map(|x| {
        let s = x.lines().collect::<Vec<&str>>();
        if s[0] == "....." {
            return Some(("key", s));
        } else {
            return Some(("lock", s));
        }
    });
    let keys = keys_locks
        .clone()
        .filter(|(k, _)| *k == "key")
        .map(|(_, v)| {
            let mut pins: Vec<u32> = vec![0, 0, 0, 0, 0];
            for (i, vv) in v.into_iter().enumerate() {
                if i == 6 {
                    continue;
                }
                for (j, c) in vv.chars().enumerate() {
                    if c == '#' {
                        pins[j] += 1;
                    }
                }
            }
            pins
        })
        .collect::<Vec<Vec<u32>>>();
    let locks = keys_locks
        .clone()
        .filter(|(k, _)| *k == "lock")
        .map(|(_, v)| {
            let mut pins: Vec<u32> = vec![0, 0, 0, 0, 0];
            for (i, vv) in v.into_iter().enumerate() {
                if i == 0 {
                    continue;
                }
                for (j, c) in vv.chars().enumerate() {
                    if c == '#' {
                        pins[j] += 1;
                    }
                }
            }
            pins
        })
        .collect::<Vec<Vec<u32>>>();

    let total = keys
        .iter()
        .cartesian_product(locks.iter())
        .map(|(k, l)| {
            let valid_pins = (0..5)
                .filter_map(|i| {
                    if k[i] + l[i] <= 5 {
                        return Some(1);
                    }
                    return None;
                })
                .sum::<u32>();
            if valid_pins == 5 {
                return 1;
            } else {
                return 0;
            }
        })
        .sum::<u32>();
    println!("Part 1: {}", total);
}
