use std::collections::HashSet;

fn dp(towels: &HashSet<&str>, design: &str) -> u64 {
    let mut d = vec![0; design.len() + 1];
    d[0] = 1;
    for i in 1..=design.len() {
        for j in 0..i {
            if d[j] > 0 && towels.contains(&design[j..i]) {
                d[i] += d[j];
            }
        }
        // println!("{} {:?}", i, d);
    }
    // println!("{:?}", d[design.len()]);
    d[design.len()]
}

pub fn main() {
    let input = std::fs::read_to_string("inputs/day19.input").unwrap();
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels: HashSet<&str> = towels.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();
    let part1 = designs.iter().filter(|d| dp(&towels, d) > 0).count();
    println!("Part 1: {}", part1);
    let part2: u64 = designs.iter().map(|d| dp(&towels, d)).sum();
    println!("Part 2: {}", part2);
}
