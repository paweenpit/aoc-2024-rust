use smallvec::SmallVec;
use std::{collections::HashMap, collections::HashSet};

struct Map(nalgebra::DMatrix<u8>);
impl Map {
    fn parse(input: &str) -> Self {
        let input = input.as_bytes();
        let width = input.iter().position(|&c| c == b'\n').unwrap();
        let mut map = nalgebra::DMatrix::from_element(width, input.len() / width, b'.');
        input.chunks(width + 1).enumerate().for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, c)| match c {
                b'\n' => {}
                _ => map[(x, y)] = *c,
            });
        });
        Self(map)
    }

    fn nodes(&self) -> HashMap<u8, SmallVec<[(usize, usize); 8]>> {
        let mut nodes = HashMap::<u8, SmallVec<[(usize, usize); 8]>>::new();
        for y in 0..self.0.nrows() {
            for x in 0..self.0.ncols() {
                let c = self.0[(x, y)];
                nodes.entry(c - b'0').or_default().push((x, y));
            }
        }
        nodes
    }
}

pub fn part1() {
    let input = std::fs::read_to_string("inputs/day10.input").unwrap();
    let map = Map::parse(&input);
    let nodes = map.nodes();
    let mut initial = HashSet::new();
    for n in &nodes[&0] {
        initial.insert(*n);
    }
    let total: u32 = initial
        .iter()
        .map(|&n| {
            // println!("------{:?}", n);
            let mut nn = HashSet::new();
            nn.insert(n);
            (0..9)
                .fold(nn, |acc: HashSet<(usize, usize)>, i| {
                    let mut new_acc = HashSet::new();
                    acc.iter().for_each(|(x, y)| {
                        if x + 1 < map.0.ncols() && map.0[(*x + 1, *y)] - b'0' == i + 1 {
                            new_acc.insert((*x + 1, *y));
                        }
                        if y + 1 < map.0.nrows() && map.0[(*x, *y + 1)] - b'0' == i + 1 {
                            new_acc.insert((*x, *y + 1));
                        }
                        if *x > 0 && map.0[(*x - 1, *y)] - b'0' == i + 1 {
                            new_acc.insert((*x - 1, *y));
                        }
                        if *y > 0 && map.0[(*x, *y - 1)] - b'0' == i + 1 {
                            new_acc.insert((*x, *y - 1));
                        }
                    });
                    // println!("{:?}", new_acc);
                    new_acc
                })
                .len() as u32
        })
        .sum();

    println!("{:?}", total);
}

pub fn part2() {
    let input = std::fs::read_to_string("inputs/day10.input").unwrap();
    let map = Map::parse(&input);
    let nodes = map.nodes();
    let mut initial = HashMap::new();
    for n in &nodes[&0] {
        initial.insert(*n, 1);
    }

    let total = (0..9).fold(initial, |acc: HashMap<(usize, usize), usize>, i| {
        let mut new_acc = HashMap::new();
        acc.iter().for_each(|((x, y), v)| {
            if x + 1 < map.0.ncols() && map.0[(*x + 1, *y)] - b'0' == i + 1 {
                *new_acc.entry((*x + 1, *y)).or_default() += v;
            }
            if y + 1 < map.0.nrows() && map.0[(*x, *y + 1)] - b'0' == i + 1 {
                *new_acc.entry((*x, *y + 1)).or_default() += v;
            }
            if *x > 0 && map.0[(*x - 1, *y)] - b'0' == i + 1 {
                *new_acc.entry((*x - 1, *y)).or_default() += v;
            }
            if *y > 0 && map.0[(*x, *y - 1)] - b'0' == i + 1 {
                *new_acc.entry((*x, *y - 1)).or_default() += v;
            }
        });
        // println!("{:?}", new_acc);
        new_acc
    });

    println!("{:?}", total.values().sum::<usize>());
}

pub fn main() {
    part1();
    part2();
}
