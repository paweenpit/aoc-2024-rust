use std::{fs, vec};

pub fn main() {
    println!("Day 04 solution!");

    // read input file line by line
    let input = fs::read_to_string("inputs/day4.input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let dir: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
        (-1, 0),
        (0, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
        (-1, -1),
    ];

    let m = "XMAS";

    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            for d in &dir {
                let mut x = i as i32;
                let mut y = j as i32;
                let mut k = 0;
                while k < m.len() {
                    if x < 0 || x >= input.len() as i32 || y < 0 || y >= input[i].len() as i32 {
                        break;
                    }
                    if input[x as usize].chars().nth(y as usize).unwrap()
                        != m.chars().nth(k).unwrap()
                    {
                        break;
                    }
                    k += 1;
                    x += d.0;
                    y += d.1;
                }
                if k == m.len() {
                    count += 1;
                }
            }
        }
    }

    println!("Part 1: {}", count);

    // Part 2
    let mut cc = 0;
    let mm: Vec<Vec<&str>> = vec![
        // "M.S\n.A.\nM.S",
        vec!["M.S", ".A.", "M.S"],
        vec!["S.M", ".A.", "S.M"],
        vec!["M.M", ".A.", "S.S"],
        vec!["S.S", ".A.", "M.M"],
    ];
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            for mmm in &mm {
                let mut found = true;
                for k in 0..mmm.len() {
                    for l in 0..mmm[k].len() {
                        if i + k >= input.len() || j + l >= input[i].len() {
                            found = false;
                            break;
                        }
                        if mmm[k].chars().nth(l).unwrap() != '.'
                            && input[i + k].chars().nth(j + l).unwrap()
                                != mmm[k].chars().nth(l).unwrap()
                        {
                            found = false;
                            break;
                        }
                    }
                    if !found {
                        break;
                    }
                }
                if found {
                    cc += 1;
                }
            }
        }
    }

    println!("Part 2: {}", cc);
}
