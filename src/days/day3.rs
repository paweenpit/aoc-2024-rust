use regex::Regex;
use std::fs;

pub fn main() {
    println!("Day 03 solution!");

    // read the input file
    let data = fs::read_to_string("inputs/day3.input").unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let total: i32 = re
        .captures_iter(&data)
        .map(|m| {
            let x: i32 = m[1].parse().unwrap();
            let y: i32 = m[2].parse().unwrap();
            x * y
        })
        .sum();

    println!("Total sum: {}", total);
}
