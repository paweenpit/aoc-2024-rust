use std::cmp::{max, min};
use std::sync::Arc;
use std::{
    collections::{HashMap, HashSet},
    vec,
};

// return shortest distances from start to all other nodes
fn bfs(
    nodes: HashSet<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
) -> HashMap<(usize, usize), i32> {
    let mut visited = HashSet::new();
    visited.insert((start.0 as i32, start.1 as i32));
    let mut queue = vec![(start, 0)];
    let mut distances: HashMap<(usize, usize), i32> = HashMap::new();
    distances.insert(start, 0);
    while !queue.is_empty() {
        let (current, steps) = queue.remove(0);
        if current == end {
            return distances;
        }
        for (i, j) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (current.0 as i32 + i, current.1 as i32 + j);
            if nodes.contains(&(new_pos.0 as usize, new_pos.1 as usize))
                && !visited.contains(&new_pos)
            {
                visited.insert(new_pos);
                distances.insert((new_pos.0 as usize, new_pos.1 as usize), steps + 1);
                queue.push(((new_pos.0 as usize, new_pos.1 as usize), steps + 1));
            }
        }
    }
    unreachable!()
}

fn get_valid_neighbors(map: &Vec<Vec<char>>, l: i32, i: i32, j: i32) -> HashMap<(i32, i32), i32> {
    let mut neighbors = HashMap::new();
    let a = map.len() as i32;
    let b = map[0].len() as i32;
    for i1 in max(0, i - l)..=min(i + l, a - 1) {
        for j1 in max(0, j - l)..=min(j + l, b - 1) {
            let dist = ((i - i1).abs() + (j - j1).abs()) as i32;
            if dist <= l
                && (map[i1 as usize][j1 as usize] == '.' || map[i1 as usize][j1 as usize] == 'E')
            {
                neighbors.insert((i1, j1), dist as i32);
            }
        }
    }
    neighbors
}

fn part1(
    map: &Vec<Vec<char>>,
    nodes: &HashSet<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
    m: usize,
    n: usize,
    cheat_len: i32,
) -> u32 {
    let start_distances = bfs(nodes.clone(), start, end);
    let end_distances = bfs(nodes.clone(), end, start);
    let max_steps = start_distances.get(&end).unwrap();
    // println!("start_distances: {:?}", start_distances);
    // println!("end_distances: {:?}", end_distances);
    println!("max_steps: {:?}", max_steps);
    let mut saved_steps: HashMap<i32, i32> = HashMap::new();
    (0..m)
        .flat_map(|i| (0..n).map(move |j| (i, j)))
        .filter(|&pos| nodes.contains(&pos))
        .for_each(|pos| {
            let (i, j) = pos;
            let valid_neighbors = get_valid_neighbors(map, cheat_len, i as i32, j as i32);
            // if pos == (start.0, start.1) {
            //     println!("valid_neighbors: {:?}", valid_neighbors);
            // }
            for ((i1, j1), dist) in valid_neighbors.into_iter() {
                let steps = start_distances.get(&(i, j)).unwrap()
                    + end_distances
                        .get(&(i1.clone() as usize, j1.clone() as usize))
                        .unwrap()
                    + dist;
                if max_steps - steps > 0 {
                    saved_steps
                        .entry(max_steps - steps)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        });
    // println!("saved_steps: {:?}", saved_steps);
    saved_steps
        .into_iter()
        .filter_map(|(k, v)| if k >= 100 { Some(v as u32) } else { Some(0) })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("inputs/day20.input").unwrap();
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nodes: HashMap<char, Vec<(usize, usize)>> =
        map.iter()
            .enumerate()
            .fold(HashMap::new(), |mut nodes, (i, row)| {
                for (j, &c) in row.iter().enumerate() {
                    nodes.entry(c).or_insert(vec![]).push((i, j));
                }
                nodes
            });
    let start = nodes.get(&'S').unwrap()[0];
    let end = nodes.get(&'E').unwrap()[0];
    let mut empty_spaces = nodes
        .get(&'.')
        .unwrap()
        .iter()
        .cloned()
        .collect::<HashSet<_>>();
    empty_spaces.insert(start);
    empty_spaces.insert(end);
    println!(
        "Part 1: {}",
        part1(&map, &empty_spaces, start, end, map.len(), map[0].len(), 2)
    );

    // part 2
    let part2 = part1(&map, &empty_spaces, start, end, map.len(), map[0].len(), 20);
    println!("Part 2: {}", part2);
}
