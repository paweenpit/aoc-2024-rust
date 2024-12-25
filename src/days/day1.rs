use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    println!("Day 01 solution!");

    // Open the input file
    let file = File::open("inputs/day1.input").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    // Read each line from the file
    for line in reader.lines() {
        let line = line.expect("Cannot read line");
        let fields: Vec<&str> = line.split_whitespace().collect();

        if fields.len() != 2 {
            println!("Skipping malformed line: {}", line);
            continue;
        }

        let num1 = fields[0].parse::<i32>();
        let num2 = fields[1].parse::<i32>();

        match (num1, num2) {
            (Ok(n1), Ok(n2)) => {
                column1.push(n1);
                column2.push(n2);
            }
            _ => println!("Skipping line with invalid numbers: {}", line),
        }
    }

    // Sort both columns
    column1.sort_unstable();
    column2.sort_unstable();

    // Part 1: Calculate the total distance
    let total_distance: i32 = column1
        .iter()
        .zip(column2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Total Distance: {}", total_distance);

    // Part 2: Similarity score
    let mut column2_count: HashMap<i32, i32> = HashMap::new();

    for &num in &column2 {
        *column2_count.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = column1
        .iter()
        .map(|num| num * column2_count.get(num).unwrap_or(&0))
        .sum();

    println!("Similarity Score: {}", similarity_score);
}
