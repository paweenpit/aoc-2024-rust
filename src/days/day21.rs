use itertools::Itertools;
use std::{cmp::min, collections::HashMap, vec};

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

    +---+---+
    | 1 | 0 |
+---+---+---+
| 4 | 3 | 2 |
+---+---+---+

(00) => A => (00)
(01) => <A => (04) + (40)
(02) => vA => (03) + (30)
(03) => v<A / <vA => (03) + (34) + (40) / (04) + (43) + (30)
(04) => v<<A / <v<A => (03) + (34) + (44) + (40) / (04) + (43) + (34) + (40)
(10) => >A => (02) + (20)
(11) => A => (00)
(12) => >vA / v>A => (02) + (23) + (30) / (03) + (32) + (20)
(13) => vA => (03) + (30)
(14) => v<A => (03) + (34) + (40)
(20) => ^A => (01) + (10)
(21) => <^A / ^<A => (04) + (41) + (10) / (01) + (14) + (40)
(22) => A => (00)
(23) => <A => (04) + (40)
(24) => <<A => (04) + (44) + (40)
(30) => ^>A / >^A => (01) + (12) + (20) / (02) + (21) + (10)
(31) => ^A => (01) + (10)
(32) => >A => (02) + (20)
(33) => A => (00)
(34) => <A => (04) + (40)
(40) => >>^A / >^>A => (02) + (22) + (21) + (10) / (02) + (21) + (12) + (20)
(41) => >^A => (02) + (21) + (10)
(42) => >>A => (02) + (22) + (20)
(43) => >A => (02) + (20)
(44) => A => (00)
*/

fn dp(n: u32) -> Vec<Vec<u64>> {
    (0..n).fold(vec![vec![1; 5]; 5], |dp, i| {
        println!("{:?}", i);
        vec![
            // 0
            vec![
                // 00
                dp[0][0],
                // 01
                dp[0][4] + dp[4][0],
                // 02
                dp[0][3] + dp[3][0],
                // 03
                min(
                    dp[0][3] + dp[3][4] + dp[4][0],
                    dp[0][4] + dp[4][3] + dp[3][0],
                ),
                // 04
                min(
                    dp[0][3] + dp[3][4] + dp[4][4] + dp[4][0],
                    dp[0][4] + dp[4][3] + dp[3][4] + dp[4][0],
                ),
            ],
            // 1
            vec![
                // 10
                dp[0][2] + dp[2][0],
                // 11
                dp[0][0],
                // 12
                min(
                    dp[0][2] + dp[2][3] + dp[3][0],
                    dp[0][3] + dp[3][2] + dp[2][0],
                ),
                // 13
                dp[0][3] + dp[3][0],
                // 14
                dp[0][3] + dp[3][4] + dp[4][0],
            ],
            // 2
            vec![
                // 20
                dp[0][1] + dp[1][0],
                // 21
                min(
                    dp[0][4] + dp[4][1] + dp[1][0],
                    dp[0][1] + dp[1][4] + dp[4][0],
                ),
                // 22
                dp[0][0],
                // 23
                dp[0][4] + dp[4][0],
                // 24
                dp[0][4] + dp[4][4] + dp[4][0],
            ],
            // 3
            vec![
                // 30
                min(
                    dp[0][1] + dp[1][2] + dp[2][0],
                    dp[0][2] + dp[2][1] + dp[1][0],
                ),
                // 31
                dp[0][1] + dp[1][0],
                // 32
                dp[0][2] + dp[2][0],
                // 33
                dp[0][0],
                // 34
                dp[0][4] + dp[4][0],
            ],
            // 4
            vec![
                // 40
                min(
                    dp[0][2] + dp[2][2] + dp[2][1] + dp[1][0],
                    dp[0][2] + dp[2][1] + dp[1][2] + dp[2][0],
                ),
                // 41
                dp[0][2] + dp[2][1] + dp[1][0],
                // 42
                dp[0][2] + dp[2][2] + dp[2][0],
                // 43
                dp[0][2] + dp[2][0],
                // 44
                dp[0][0],
            ],
        ]
    })
}

fn filter_validate_paths(k1: (u32, u32), paths: Vec<Vec<char>>) -> Vec<Vec<char>> {
    paths
        .into_iter()
        .filter(|p| {
            let mut current = k1.clone();
            for c in p.iter() {
                match c {
                    '^' => current.0 -= 1,
                    'v' => current.0 += 1,
                    '<' => current.1 -= 1,
                    '>' => current.1 += 1,
                    _ => panic!("Invalid move"),
                }
                if current == (3, 0) {
                    return false;
                }
            }
            true
        })
        .collect()
}

fn get_shortest_paths(k1: (u32, u32), k2: (u32, u32)) -> Vec<Vec<char>> {
    let mut path = Vec::new();
    let mut current = k1.clone();
    while current != k2 {
        if current.1 < k2.1 {
            path.push('>');
            current.1 += 1;
        } else if current.0 > k2.0 {
            path.push('^');
            current.0 -= 1;
        } else if current.0 < k2.0 {
            path.push('v');
            current.0 += 1;
        } else if current.1 > k2.1 {
            path.push('<');
            current.1 -= 1;
        }
    }
    let path_len = path.len();
    let mut all_paths: Vec<Vec<char>> = path.into_iter().permutations(path_len).unique().collect();
    // validate all paths not contain (3,0)
    all_paths = filter_validate_paths(k1, all_paths);

    // append 'A' to all path in paths
    all_paths.iter_mut().for_each(|p| p.push('A'));
    all_paths
}

fn convert_path(path: Vec<char>) -> Vec<(u32, u32)> {
    let x: Vec<char> = vec!['A', '^', '>', 'v', '<'];
    let mut m = HashMap::new();
    for i in 0..5 {
        for j in 0..5 {
            m.insert((x[i], x[j]), (i as u32, j as u32));
        }
    }
    let mut p = vec!['A'];
    p.extend(path.iter());

    let mut result: Vec<(u32, u32)> = vec![];
    p.windows(2)
        .map(|pair| m.get(&(pair[0], pair[1])).unwrap())
        .for_each(|(i, j)| result.push((*i, *j)));
    result
}

pub fn main() {
    let n = 25;
    let dp = dp(n);
    let nodes = vec![
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
        (3, 1),
        (3, 2),
    ];
    let mut costs: HashMap<((u32, u32), (u32, u32)), u64> = HashMap::new();
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            let paths = get_shortest_paths(nodes[i], nodes[j]);
            let converted_paths = paths.iter().map(|p| convert_path(p.clone())).collect_vec();
            let cost = converted_paths
                .iter()
                .map(|p| {
                    p.iter()
                        .map(|(i, j)| dp[*i as usize][*j as usize])
                        .sum::<u64>()
                })
                .min()
                .unwrap();
            costs.insert((nodes[i], nodes[j]), cost as u64);
        }
    }

    let input = std::fs::read_to_string("inputs/day21.input").expect("could not read day21 input");
    let total2 = input
        .lines()
        .map(|l| {
            let mut x = vec!['A'];
            x.extend(l.chars());
            let y = x.iter().map(|c| convert_numpad_to_loc(*c)).collect_vec();
            let cost = y
                .windows(2)
                .map(|w| costs.get(&(w[0], w[1])).unwrap())
                .sum::<u64>();
            let num = l[..l.len() - 1].parse::<u64>().unwrap();
            cost * num
        })
        .sum::<u64>();
    println!("{:?}", total2);
}

fn convert_numpad_to_loc(c: char) -> (u32, u32) {
    match c {
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => panic!("Invalid move"),
    }
}
