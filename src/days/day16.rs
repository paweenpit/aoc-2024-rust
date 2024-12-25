use priority_queue::PriorityQueue;
use std::{
    collections::{HashMap, HashSet},
    vec,
};

fn neighbors(
    nodes: &HashMap<char, Vec<(usize, usize)>>,
    n: (usize, usize),
) -> Option<Vec<(usize, usize, i32)>> {
    Some(
        nodes
            .get(&'.')
            .unwrap()
            .iter()
            .filter_map(|(i, j)| {
                // (i,j) - (n.0, n.1) == (0, 1) || (1, 0) || (-1, 0) || (0, -1)
                if (*i as isize - n.0 as isize).abs() + (*j as isize - n.1 as isize).abs() == 1 {
                    let dir = match (*i as isize - n.0 as isize, *j as isize - n.1 as isize) {
                        (0, 1) => 1,
                        (-1, 0) => 2,
                        (0, -1) => 3,
                        (1, 0) => 4,
                        _ => unreachable!(),
                    };
                    // println!("dir: {}", dir);
                    Some((*i, *j, dir))
                } else {
                    None
                }
            })
            .collect(),
    )
}

fn diajkstra(
    nodes: &HashMap<char, Vec<(usize, usize)>>,
    start: (usize, usize),
) -> (
    HashMap<(usize, usize, i32), i32>,
    HashMap<(usize, usize, i32), Vec<Vec<(usize, usize)>>>,
) {
    // let mut visited: HashSet<(usize, usize, i32)> = HashSet::new();
    let mut dist: HashMap<(usize, usize, i32), i32> = HashMap::new(); // (i, j, dir) -> dist (dir = (>, ^, <, v) == (1,2,3,4))
    let mut paths: HashMap<(usize, usize, i32), Vec<Vec<(usize, usize)>>> = HashMap::new(); // (i, j, dir) -> [path1, path2, ...]
    dist.insert((start.0, start.1, 1), 0);
    paths.insert((start.0, start.1, 1), vec![vec![start]]);
    let mut queue: PriorityQueue<(i32, (usize, usize), i32), i32> = PriorityQueue::new(); // (d, (i, j), dir), d
    queue.push((0, start, 1), 0);
    while let Some(((d, (i, j), dir), _)) = queue.pop() {
        if let Some(neighbors) = neighbors(nodes, (i, j)) {
            for (ni, nj, new_dir) in neighbors {
                if (dir - new_dir).abs() == 2 {
                    continue;
                }
                let nd = dist[&(i, j, dir)] + 1 + if dir == new_dir { 0 } else { 1000 };
                let old_d = dist
                    .get(&(ni, nj, new_dir))
                    .cloned()
                    .unwrap_or(std::i32::MAX);
                if nd <= old_d {
                    dist.insert((ni, nj, new_dir), nd);
                    queue.push((nd, (ni, nj), new_dir), -nd);
                    let mut new_path = paths[&(i, j, dir)].clone();
                    let cur_path = paths.get(&(ni, nj, new_dir)).cloned().unwrap_or(vec![]);
                    // if same score => add to path
                    if nd == old_d {
                        new_path.extend(cur_path);
                    }
                    new_path.iter_mut().for_each(|p| p.push((ni, nj)));
                    // println!("new_path: {:?}", new_path);
                    paths.insert((ni, nj, new_dir), new_path);
                    // std::io::stdin().read_line(&mut String::new()).unwrap();
                }
            }
        }
        // println!("queue {:?}", queue.clone().into_vec());
        // std::io::stdin().read_line(&mut String::new()).unwrap();
    }
    (dist, paths)
}

pub fn main() {
    let input = std::fs::read_to_string("inputs/day16.input").unwrap();
    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            nodes.entry(c).or_default().push((i, j));
        })
    });
    // println!("nodes: {:?}", nodes);
    let start = nodes[&'S'][0];
    let end = nodes[&'E'][0];
    nodes.entry('.').or_insert(vec![]).push(end);
    let (dist, paths) = diajkstra(&nodes, start);
    // println!("dist: {:?}", dist);
    let mut min_dist = std::i32::MAX;
    for i in 1..5 {
        if let Some(&d) = dist.get(&(end.0, end.1, i)) {
            println!("d: {}", d);
            min_dist = min_dist.min(d);
        }
    }
    // NOTE: part 2 only work with 1 final direction to reach the end
    let nodes_in_paths =
        paths
            .get(&(end.0, end.1, 2))
            .unwrap()
            .into_iter()
            .fold(HashSet::new(), |mut acc, p| {
                p.iter().for_each(|(i, j)| {
                    acc.insert((*i, *j));
                });
                acc
            });
    println!("min_dist: {}", min_dist);
    println!("nodes_in_paths len: {}", nodes_in_paths.len());
}
