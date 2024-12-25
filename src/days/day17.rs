use anyhow::anyhow;
use itertools::Itertools;

fn combo(c: u64, registers: &Vec<u64>) -> u64 {
    if 4 <= c && c <= 6 {
        return registers[c as usize - 4];
    }
    if c == 7 {
        unreachable!("c == 7");
    }
    c
}

// returns: (x, y)
// x = 0 => nothing, y = 0
// x = 1 => output, y = value
// x = 2 => jump, y = value
// x = 3 => do nothing, y = 0, index += 1
// x = 4 => exit program, y = 0
fn perform(index: &u64, registers: &mut Vec<u64>, program: &Vec<u64>) -> (u64, u64) {
    let opcode = program[*index as usize];
    if opcode == 3 && registers[0] == 0 {
        return (3, 0);
    }
    if *index + 1 >= program.len() as u64 {
        return (4, 0);
    }
    let operand = program[*index as usize + 1];
    match opcode {
        0 => {
            registers[0] = registers[0] / 2u64.pow(combo(operand, registers) as u32);
        }
        1 => {
            registers[1] = registers[1] ^ operand;
        }
        2 => {
            registers[1] = combo(operand, registers) % 8;
        }
        3 => {
            if registers[0] != 0 {
                return (2, operand);
            }
        }
        4 => {
            registers[1] = registers[1] ^ registers[2];
        }
        5 => return (1, combo(operand, registers) % 8),
        6 => {
            registers[1] = registers[0] / 2u64.pow(combo(operand, registers) as u32);
        }
        7 => {
            registers[2] = registers[0] / 2u64.pow(combo(operand, registers) as u32);
        }
        _ => unreachable!(),
    }
    return (0, 0);
}

pub fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day17.input")?;
    let input = input.split_once("\n\n").ok_or(anyhow!("Invalid input"))?;
    let registers = input
        .0
        .lines()
        .map(|l| {
            Ok(l.split_once(": ")
                .ok_or(anyhow!("split error"))?
                .1
                .parse::<u64>()?)
        })
        .collect::<anyhow::Result<Vec<u64>>>()?;
    let mut registers1 = registers.clone();
    println!("{:?}", registers1);
    let program = input
        .1
        .split_once(": ")
        .ok_or(anyhow!("split error"))?
        .1
        .split(',')
        .map(|s| Ok(s.parse::<u64>()?))
        .collect::<anyhow::Result<Vec<u64>>>()?;
    println!("{:?}", program);
    let mut index = 0u64;
    let mut output: Vec<u64> = vec![];
    loop {
        let (action, value) = perform(&index, &mut registers1, &program);
        match action {
            0 => {
                index += 2;
            }
            1 => {
                index += 2;
                output.push(value);
            }
            2 => {
                index = value;
            }
            3 => {
                index += 1;
            }
            4 => break,
            _ => {}
        }
        if index as usize >= program.len() {
            break;
        }
        println!("{} {} {}", action, value, index);
        println!("{:?}", registers1);
        // std::io::stdin().read_line(&mut String::new())?;
    }
    println!("Register: {:?}", registers1);
    println!("Output: {:?}", output);
    println!("Part 1: {}", output.into_iter().join(","));

    // Part 2
    let mut new_a = 100000000u64;
    loop {
        // new_a += 2u64.pow(6);
        new_a += 1;
        if new_a % 1000 == 0 {
            println!("{}", new_a);
        }
        let mut index2 = 0u64;
        let mut registers2 = registers.clone();
        registers2[0] = new_a;
        let mut output: Vec<u64> = vec![];
        let mut found = false;
        loop {
            let (action, value) = perform(&index2, &mut registers2, &program);
            match action {
                0 => {
                    index2 += 2;
                }
                1 => {
                    index2 += 2;
                    output.push(value);
                }
                2 => {
                    index2 = value;
                }
                3 => {
                    index2 += 1;
                }
                4 => break,
                _ => {}
            }
            if index2 as usize >= program.len() {
                break;
            }
            if output == program {
                found = true;
                println!("Part 2: {}", new_a);
                break;
            }
            // if output.len() == program.len()
            //     && output[output.len() - 14..] == program[program.len() - 14..]
            // {
            //     println!("{} {:?}", new_a, output);
            //     std::io::stdin().read_line(&mut String::new())?;
            //     break;
            // }
        }
        println!("{} {:?} {}", new_a, output, output.len());
        if found {
            break;
        }
        // std::io::stdin().read_line(&mut String::new())?;
    }

    Ok(())
}
