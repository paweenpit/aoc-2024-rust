fn is_solvable_part1(
    mut eq_operands: impl std::iter::Iterator<Item = u64> + Clone,
    eq_result: u64,
    intermediate_result: u64,
    i: u64,
) -> bool {
    if let Some(next_operand) = eq_operands.next() {
        if i == 0 {
            is_solvable_part1(
                eq_operands.clone(),
                eq_result,
                intermediate_result + next_operand,
                i + 1,
            )
        } else {
            is_solvable_part1(
                eq_operands.clone(),
                eq_result,
                intermediate_result + next_operand,
                i + 1,
            ) || is_solvable_part1(
                eq_operands,
                eq_result,
                intermediate_result * next_operand,
                i + 1,
            )
        }
    } else {
        eq_result == intermediate_result
    }
}

fn is_solvable_part2(
    mut eq_operands: impl std::iter::Iterator<Item = u64> + Clone,
    eq_result: u64,
    intermediate_result: u64,
    i: u64,
) -> bool {
    if let Some(next_operand) = eq_operands.next() {
        if i == 0 {
            is_solvable_part2(
                eq_operands.clone(),
                eq_result,
                intermediate_result + next_operand,
                i + 1,
            )
        } else {
            is_solvable_part2(
                eq_operands.clone(),
                eq_result,
                intermediate_result + next_operand,
                i + 1,
            ) || is_solvable_part2(
                eq_operands.clone(),
                eq_result,
                intermediate_result * next_operand,
                i + 1,
            ) || is_solvable_part2(
                eq_operands.clone(),
                eq_result,
                (intermediate_result.to_string() + next_operand.to_string().as_str())
                    .parse()
                    .unwrap(),
                i + 1,
            )
        }
    } else {
        eq_result == intermediate_result
    }
}

pub fn main() {
    let input = std::fs::read_to_string("inputs/day7.input").unwrap();

    let total: u64 = input
        .lines()
        .filter_map(|line| {
            let (eq_result, eq_operands) = line.split_once(": ")?;
            let result = eq_result.parse().unwrap();
            let eq_operands = eq_operands
                .split_whitespace()
                .filter_map(|x| x.parse().ok());
            is_solvable_part1(eq_operands, result, 0, 0).then_some(result)
        })
        .sum();

    println!("Part1: {}", total);

    let total2: u64 = input
        .lines()
        .filter_map(|line| {
            let (eq_result, eq_operands) = line.split_once(": ")?;
            let result = eq_result.parse().unwrap();
            let eq_operands = eq_operands
                .split_whitespace()
                .filter_map(|x| x.parse().ok());
            is_solvable_part2(eq_operands, result, 0, 0).then_some(result)
        })
        .sum();

    println!("Part2: {}", total2);
}
