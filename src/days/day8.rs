use std::collections::{HashMap, HashSet};

pub fn main() {
    let mut loc: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let map: Vec<Vec<char>> = std::fs::read_to_string("inputs/day8.input")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    loc.entry(c)
                        .or_insert(Vec::new())
                        .push((x as isize, y as isize));
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect();

    let m = map.len();
    let n = map[0].len();

    let mut antennas: HashSet<(usize, usize)> = HashSet::new();

    let _: Vec<_> = loc
        .iter()
        .filter_map(|(c, v)| {
            if *c != '.' {
                let _: Vec<_> = v
                    .iter()
                    .enumerate()
                    .flat_map(|(i, &x)| v.iter().skip(i + 1).map(move |&y| (x, y)))
                    .map(|(a, b)| {
                        let n1 = (2 * a.0 - b.0, 2 * a.1 - b.1);
                        let n2 = (2 * b.0 - a.0, 2 * b.1 - a.1);
                        if n1.0 >= 0 && n1.0 < m as isize && n1.1 >= 0 && n1.1 < n as isize {
                            antennas.insert((n1.0 as usize, n1.1 as usize));
                        }
                        if n2.0 >= 0 && n2.0 < m as isize && n2.1 >= 0 && n2.1 < n as isize {
                            antennas.insert((n2.0 as usize, n2.1 as usize));
                        }
                    })
                    .collect();
                Some(())
            } else {
                None
            }
        })
        .collect();

    println!("Part 1: {}", antennas.len());

    let mut antennas2: HashSet<(usize, usize)> = HashSet::new();

    loc.iter().for_each(|(c, v)| {
        if *c != '.' && v.len() > 1 {
            v.iter()
                .enumerate()
                .flat_map(|(i, &x)| v.iter().skip(i + 1).map(move |&y| (x, y)))
                .for_each(|(a, b)| {
                    antennas2.insert((a.0 as usize, a.1 as usize));
                    antennas2.insert((b.0 as usize, b.1 as usize));
                    let mut i = 2;
                    loop {
                        let n1 = (i * a.0 - (i - 1) * b.0, i * a.1 - (i - 1) * b.1);
                        if n1.0 < 0 || n1.0 >= m as isize || n1.1 < 0 || n1.1 >= n as isize {
                            break;
                        }
                        antennas2.insert((n1.0 as usize, n1.1 as usize));
                        i += 1;
                    }
                    let mut j = 2;
                    loop {
                        let n2 = (j * b.0 - (j - 1) * a.0, j * b.1 - (j - 1) * a.1);
                        if n2.0 < 0 || n2.0 >= m as isize || n2.1 < 0 || n2.1 >= n as isize {
                            break;
                        }
                        antennas2.insert((n2.0 as usize, n2.1 as usize));
                        j += 1;
                    }
                });
        }
    });

    println!("Part 2: {}", antennas2.len());
}
