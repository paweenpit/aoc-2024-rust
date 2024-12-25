use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn area(coords: &Vec<Point>) -> i32 {
    coords.len() as i32
}

fn neighbor(p1: Point, p2: Point) -> bool {
    (p1.x == p2.x && p1.y == p2.y + 1)
        || (p1.x == p2.x && p1.y + 1 == p2.y)
        || (p1.x == p2.x + 1 && p1.y == p2.y)
        || (p1.x + 1 == p2.x && p1.y == p2.y)
}

// edge is adjacent
fn neighbor2(p1: Point, p2: Point, points: &Vec<Point>) -> i32 {
    let mut total = 0;
    for dx in [-1, 1] {
        for dy in [-1, 1] {
            if p1.x + dx == p2.x
                && p1.y == p2.y
                && !points.contains(&Point {
                    x: p1.x,
                    y: p1.y + dy,
                })
                && !points.contains(&Point {
                    x: p2.x,
                    y: p2.y + dy,
                })
            {
                total += 1;
            }
            if p1.x == p2.x
                && p1.y + dy == p2.y
                && !points.contains(&Point {
                    x: p1.x + dx,
                    y: p1.y,
                })
                && !points.contains(&Point {
                    x: p2.x + dx,
                    y: p2.y,
                })
            {
                total += 1;
            }
        }
    }
    total
}

fn perimeter(coords: &Vec<Point>) -> i32 {
    let mut t = (4 * coords.len()) as i32;
    (0..coords.len()).for_each(|i| {
        ((i + 1)..coords.len()).for_each(|j| {
            if neighbor(coords[i], coords[j]) {
                t -= 2i32;
            }
        })
    });
    t
}

fn perimeter2(coords: &Vec<Point>) -> i32 {
    let mut t = (4 * coords.len()) as i32;
    (0..coords.len()).for_each(|i| {
        ((i + 1)..coords.len()).for_each(|j| {
            if neighbor(coords[i], coords[j]) {
                t -= 2i32;
            }
            t -= neighbor2(coords[i], coords[j], coords);
        })
    });
    t
}

fn get_islands(points: Vec<Point>) -> Vec<Vec<Point>> {
    let mut islands: Vec<Vec<Point>> = Vec::new();
    let mut points_found: HashSet<Point> = HashSet::new();
    points.iter().for_each(|p| {
        if !points_found.contains(p) {
            let mut island: Vec<Point> = Vec::new();
            let mut stack: Vec<Point> = vec![*p];
            while !stack.is_empty() {
                let current = stack.pop().unwrap();
                if !points_found.contains(&current) {
                    points_found.insert(current);
                    island.push(current);
                    let neighbors: Vec<Point> = points
                        .iter()
                        .filter(|p| neighbor(current, **p))
                        .map(|p| *p)
                        .collect();
                    stack.extend(neighbors);
                }
            }
            islands.push(island);
        }
    });
    islands
}

pub fn main() {
    let input = std::fs::read_to_string("inputs/day12.input").unwrap();
    let map: HashMap<char, Vec<Point>> =
        input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, line)| {
                line.chars().enumerate().for_each(|(j, c)| {
                    acc.entry(c).or_insert(Vec::new()).push(Point {
                        x: j as i32,
                        y: i as i32,
                    });
                });
                acc
            });
    let map_island: HashMap<char, Vec<Vec<Point>>> = map
        .iter()
        .map(|(c, v)| (*c, get_islands(v.clone())))
        .collect();

    let total: i32 = map_island
        .iter()
        .map(|(_, islands)| {
            islands
                .iter()
                .map(|island| area(island) * perimeter(island))
                .sum::<i32>()
        })
        .sum();
    println!("Part 1 {total}");

    let total2: i32 = map_island
        .iter()
        .map(|(_, islands)| {
            islands
                .iter()
                .map(|island| area(island) * perimeter2(island))
                .sum::<i32>()
        })
        .sum();
    println!("Part 2: {total2}");
}
