use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn print_map(nodes: &HashSet<(u32, u32)>, blocks: &HashMap<u32, (u32, u32)>) {
    for y in 0..70 {
        for x in 0..70 {
            if blocks.values().any(|&v| v == (x, y)) {
                print!("#");
            } else if nodes.contains(&(x, y)) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn bfs(nodes: HashSet<(u32, u32)>, start: (u32, u32), end: (u32, u32)) -> i32 {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push((start.0, start.1, 0));
    while !queue.is_empty() {
        let (x, y, steps) = queue.remove(0);
        if (x, y) == end {
            return steps;
        }
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if new_x < 0 || new_y < 0 {
                continue;
            }
            if !nodes.contains(&(new_x as u32, new_y as u32)) {
                continue;
            }
            queue.push((new_x as u32, new_y as u32, steps + 1));
        }
    }
    -1
}

fn part1(blocks: &HashMap<u32, (u32, u32)>, i: u32) -> i32 {
    let filtered_blocks: HashMap<_, _> = blocks
        .clone()
        .into_iter()
        .filter(|(k, _)| *k <= i)
        .collect();
    // convert blocks values to a HashSet
    let block_locs = filtered_blocks.values().cloned().collect::<HashSet<_>>();
    // Create a 2D array of nodes with 70x70 dimensions that do not contain blocks_locs
    let nodes = (0..=70)
        .flat_map(|y| (0..=70).map(move |x| (x, y)))
        .filter(|&loc| !block_locs.contains(&loc))
        .collect::<HashSet<_>>();

    // print_map(&nodes, &filtered_blocks);
    bfs(nodes.clone(), (0, 0), (70, 70))
}

pub fn main() {
    let input = std::fs::read_to_string("inputs/day18.input").unwrap();
    let blocks: HashMap<u32, (u32, u32)> = input
        .lines() // Split input into lines
        .enumerate() // Add 1-based index
        .map(|(i, line)| {
            let (a, b) = line
                .split_once(',') // Split each line by ','
                .expect("Each line must contain a comma");
            (
                (i + 1) as u32, // Convert index to 1-based
                (a.parse().unwrap(), b.parse().unwrap()),
            )
        })
        .collect();

    println!("Part 1: {}", part1(&blocks, 1024));

    let mut v = (0, blocks.len() - 1);
    loop {
        if v.0 >= v.1 {
            break;
        }
        let mid = (v.0 + v.1) / 2;
        let steps = part1(&blocks, mid as u32);
        println!("{}: {} {:?}", mid, steps, blocks[&(mid as u32)]);
        if steps == -1 {
            v.1 = mid;
        } else {
            v.0 = mid + 1;
        }
    }
    println!("Part 2: {:?}", blocks[&(v.0 as u32)]);
}
