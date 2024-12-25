use std::cmp::Ordering;
use std::fs;

pub fn main() {
    println!("Day 05 solution!");

    let file_content = fs::read_to_string("inputs/day5.input").unwrap();
    let input: Vec<&str> = file_content.split("\n\n").collect();

    let orders: Vec<(i32, i32)> = input[0]
        .lines()
        .filter_map(|line| {
            line.split_once("|")
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        })
        .collect();
    let books: Vec<Vec<i32>> = input[1]
        .lines()
        .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();

    let total: i32 = books
        .iter()
        .map(|book| {
            book.is_sorted_by(|a, b| !orders.contains(&(*b, *a)))
                .then_some(book[book.len() / 2])
                .unwrap_or(0)
        })
        .sum();

    println!("Part 1: {}", total);

    let mut mut_book = books.clone();
    let total2: i32 = mut_book
        .iter_mut()
        .map(|book| {
            (!book.is_sorted_by(|a, b| !orders.contains(&(*b, *a))))
                .then_some({
                    book.sort_by(|a, b| {
                        if orders.contains(&(*a, *b)) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    });
                    book[book.len() / 2]
                })
                .unwrap_or(0)
        })
        .sum();

    println!("Part 2: {:?}", total2);
}
