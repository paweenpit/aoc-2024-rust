use itertools::{iproduct, Itertools};
use regex::Regex;
use std::collections::HashMap;

pub fn main() {
    let input = std::fs::read_to_string("inputs/day24.input").expect("Can't read file");
    let parts = input.split_once("\n\n").expect("Invalid input");
    let re1 = Regex::new(r"([a-z]\d+): (\d+)").unwrap();
    let wires: HashMap<&str, u32> = parts
        .0
        .lines()
        .map(|line| {
            let x = re1.captures(line).unwrap();
            (
                x.get(1).unwrap().as_str(),
                x.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            )
        })
        .collect();
    let re2 = Regex::new(r"([a-z0-9]{3}) ([A-Z0-9]+) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();
    let gates: Vec<_> = parts
        .1
        .lines()
        .map(|line| {
            let x = re2.captures(line).unwrap();
            (
                x.get(1).unwrap().as_str(),
                x.get(2).unwrap().as_str(),
                x.get(3).unwrap().as_str(),
                x.get(4).unwrap().as_str(),
            )
        })
        .collect();
    let mut gates_order = gates.clone();
    // let mut wires1 = wires.clone();
    // let mut gates1 = gates.clone();
    // let mut gates_order = vec![];
    // loop {
    //     let mut to_remove_index = vec![];
    //     for (i, (w1, op, w2, r)) in gates1.clone().into_iter().enumerate() {
    //         println!("{:?}", i);
    //         if let (Some(v1), Some(v2)) = (wires1.get(w1), wires1.get(w2)) {
    //             let res = match op {
    //                 "AND" => v1 & v2,
    //                 "OR" => v1 | v2,
    //                 "XOR" => v1 ^ v2,
    //                 _ => panic!("Invalid operation"),
    //             };
    //             wires1.insert(r, res);
    //             to_remove_index.push(i);
    //             gates_order.push((w1, op, w2, r));
    //         }
    //     }
    //     for i in to_remove_index.into_iter().rev() {
    //         gates1.remove(i);
    //     }
    //     if gates1.is_empty() {
    //         break;
    //     }
    // }
    // let (resz, totalz) = get_bin_value('z', &wires1);
    // println!("Part 1: {}", totalz);

    // part 2
    swap_gates(&mut gates_order, "qgd", "z18");
    swap_gates(&mut gates_order, "mwk", "z10");
    swap_gates(&mut gates_order, "jmh", "hsw");
    swap_gates(&mut gates_order, "z33", "gqp");
    // println!("{:?}", gates_order);

    let combinations: Vec<[u32; 6]> = iproduct!(0..2, 0..2, 0..2, 0..2, 0..2, 0..2)
        .map(|(a, b, c, d, e, f)| [a, b, c, d, e, f])
        .collect();

    for c in combinations {
        let mut w = wires.clone();
        w.insert(&"x00", c[0]);
        w.insert(&"x01", c[1]);
        w.insert(&"x02", c[2]);
        w.insert(&"y00", c[3]);
        w.insert(&"y01", c[4]);
        w.insert(&"y02", c[5]);
        // println!(
        //     "{:?}",
        //     w.iter()
        //         .filter(|(k, _)| ["x00", "x01", "x02", "y00", "y01", "y02"].contains(k))
        //         .collect::<Vec<_>>()
        // );
        let wires2 = compute(&w, &gates_order);
        let (resx, totalx) = get_bin_value('x', &wires2);
        let (resy, totaly) = get_bin_value('y', &wires2);
        let (resz, totalz) = get_bin_value('z', &wires2);
        // // println!("{} {} {}", totalx, totaly, totalx + totaly);
        // println!(" 432109876543210987654321098765432109876543210");
        // println!(" {}\n {}\n{}", resx, resy, resz);
        // println!("{}", totalx + totaly - totalz);

        let res2 = vec!["qgd", "mwk", "jmh", "z33", "z18", "z10", "hsw", "gqp"];
        let total2 = res2.iter().sorted().join(",").to_string();
        if totalx + totaly - totalz != 0 {
            println!("Part 2: {}", totalx + totaly - totalz);
        }
        // println!("Part 2: {}", total2);
    }

    let wires2 = compute(&wires, &gates_order);
    let (resx, totalx) = get_bin_value('x', &wires2);
    let (resy, totaly) = get_bin_value('y', &wires2);
    let (resz, totalz) = get_bin_value('z', &wires2);
    // println!("{} {} {}", totalx, totaly, totalx + totaly);
    println!(" 432109876543210987654321098765432109876543210");
    println!(" {}\n {}\n{}", resx, resy, resz);
    println!("{}", totalx + totaly - totalz);

    let res2 = vec!["qgd", "mwk", "jmh", "z33", "z18", "z10", "hsw", "gqp"];
    let total2 = res2.iter().sorted().join(",").to_string();
    println!("Part 2: {}", total2);
}

fn print_zn(n: u32, orders: &Vec<(&str, &str, &str, &str)>) {
    for (i, (w1, op, w2, r)) in orders.into_iter().enumerate() {
        if *r == "z".to_owned() + &n.to_string().as_str() {
            println!("{} {} {} {} {}", i, w1, op, w2, r);
        }
    }
}

fn get_bin_value(c: char, wires: &HashMap<&str, u32>) -> (String, i64) {
    let resy = wires
        .clone()
        .into_iter()
        .sorted()
        .rev()
        .filter(|(k, v)| k.starts_with(c))
        .map(|(k, v)| v)
        .collect::<Vec<u32>>()
        .iter()
        .join("");
    // println!("{:?}", resy);
    (resy.clone(), i64::from_str_radix(&resy, 2).unwrap())
}

fn swap_gates<'a>(gates: &mut Vec<(&'a str, &'a str, &'a str, &'a str)>, a: &'a str, b: &'a str) {
    for (_, _, _, r) in gates.iter_mut() {
        if *r == a {
            *r = b;
        } else if *r == b {
            *r = a;
        }
    }
}

fn compute<'a>(
    wires: &'a HashMap<&'a str, u32>,
    gates: &'a Vec<(&'a str, &'a str, &'a str, &'a str)>,
) -> HashMap<&'a str, u32> {
    let mut wires1 = wires.clone();
    let mut gates1 = gates.clone();
    let mut gates_order = vec![];
    loop {
        let mut to_remove_index = vec![];
        for (i, (w1, op, w2, r)) in gates1.clone().into_iter().enumerate() {
            if let (Some(v1), Some(v2)) = (wires1.get(w1), wires1.get(w2)) {
                let res = match op {
                    "AND" => v1 & v2,
                    "OR" => v1 | v2,
                    "XOR" => v1 ^ v2,
                    _ => panic!("Invalid operation"),
                };
                wires1.insert(r, res);
                to_remove_index.push(i);
                gates_order.push((w1, op, w2, r));
            }
        }
        for i in to_remove_index.into_iter().rev() {
            gates1.remove(i);
        }
        if gates1.is_empty() {
            break;
        }
    }
    wires1
}
