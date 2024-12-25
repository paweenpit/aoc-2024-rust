fn foo(x1: i64, y1: i64, x2: i64, y2: i64, t1: i64, t2: i64) -> i64 {
    // find a1, a2 such that
    // x1 * a1 + x2 * a2 = t1
    // y1 * a1 + y2 * a2 = t2
    // min 3*a1 + a2
    // a2 = (t1 * y1 - t2 * x1) / (x2 * y1 - x1 * y2)
    let c2 = t1 * y1 - t2 * x1;
    let d2 = x2 * y1 - x1 * y2;
    if d2 == 0 {
        if t1 * y1 != t2 * x1 {
            return 0;
        }
        for i in 0..(t1 / x1) {
            if (t1 - i * x1) % x2 == 0 {
                return 3 * i + (t1 - i * x1) / x2;
            }
        }
    }
    if c2 % d2 != 0 {
        return 0;
    }
    let a2 = c2 / d2;
    let c1 = t1 - x2 * a2;
    let d1 = x1;
    if c1 % d1 != 0 {
        return 0;
    }
    let a1 = c1 / d1;
    return 3 * a1 + a2;
}

pub fn main() {
    let re = regex::Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let input = std::fs::read_to_string("inputs/day13.input").unwrap();
    let caps: Vec<Vec<i64>> = re
        .captures_iter(&input)
        .map(|cap| {
            cap.iter()
                .skip(1)
                .map(|x| x.unwrap().as_str().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let total: i64 = caps
        .iter()
        .map(|x| foo(x[0], x[1], x[2], x[3], x[4], x[5]))
        .sum();

    println!("Part 1: {}", total);

    let total2: i64 = caps
        .iter()
        .map(|x| {
            foo(
                x[0],
                x[1],
                x[2],
                x[3],
                x[4] + 10000000000000i64,
                x[5] + 10000000000000i64,
            )
        })
        .sum();

    println!("Part 2: {}", total2);
}
