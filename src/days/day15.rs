use std::collections::HashSet;

fn mv(map: &mut Vec<Vec<char>>, x: i32, y: i32, dx: i32, dy: i32) -> (i32, i32) {
    let mut xx = x;
    let mut yy = y;
    let mut blocks: Vec<(i32, i32)> = vec![(x, y)];
    loop {
        xx += dx;
        yy += dy;
        let c = map[xx as usize][yy as usize];
        match c {
            '#' => return (x, y),
            '.' => {
                blocks.push((xx, yy));
                // move blocks
                let mut chars: Vec<char> = blocks
                    .iter()
                    .map(|(a, b)| map[*a as usize][*b as usize])
                    .collect();
                chars.rotate_right(1);
                for (i, (a, b)) in blocks.iter().enumerate() {
                    map[*a as usize][*b as usize] = chars[i];
                }

                return (blocks[1].0, blocks[1].1);
            }
            'O' => {
                blocks.push((xx, yy));
            }
            _ => println!("error"),
        }
    }
}

fn mv2(map: &mut Vec<Vec<char>>, x: i32, y: i32, dx: i32, dy: i32) -> (i32, i32) {
    // println!("mv2 {} {} {} {}", x, y, dx, dy);
    let mut blocks: Vec<Vec<(i32, i32)>> = vec![vec![(x, y)]];
    loop {
        // println!("1");
        let last_blocks = blocks.last().unwrap();
        // can move
        // println!("2");
        if last_blocks
            .iter()
            .all(|(a, b)| map[*a as usize][*b as usize] == '.')
        {
            // println!("blocks {:?}", blocks);
            for block in blocks.iter().rev() {
                // println!("block {:?}", block);
                for (a, b) in block.iter() {
                    // println!("a b {} {}", a, b);
                    let xx = a + dx;
                    let yy = b + dy;
                    map[xx as usize][yy as usize] = map[*a as usize][*b as usize];
                    // println!("setting {} {} to {}", xx, yy, map[*a as usize][*b as usize]);
                    map[*a as usize][*b as usize] = '.';
                    // println!("setting {} {} to .", a, b,);
                }
            }
            map[x as usize][y as usize] = '.';
            return (x + dx, y + dy);
        };
        // println!("3");
        let mut new_blocks: HashSet<(i32, i32)> = HashSet::new();
        for (a, b) in last_blocks.iter() {
            let xx = a + dx;
            let yy = b + dy;
            let c = map[xx as usize][yy as usize];
            match c {
                '#' => return (x, y),
                '.' => {}
                '[' => {
                    new_blocks.insert((xx, yy));
                    if dx != 0 {
                        new_blocks.insert((xx, yy + 1));
                    }
                }
                ']' => {
                    new_blocks.insert((xx, yy));
                    if dx != 0 {
                        new_blocks.insert((xx, yy - 1));
                    }
                }
                _ => println!("error {}", c),
            }
        }
        // println!("4");
        blocks.push(new_blocks.into_iter().collect());
        // println!("5");
    }
}

fn print_map(map: &mut Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{}", map[i as usize][j as usize]);
        }
        println!();
    }
}

fn find_robot(map: &mut Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '@' {
                return (i as i32, j as i32);
            }
        }
    }
    return (0, 0);
}

fn find_blocks(map: &mut Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut blocks: Vec<(i32, i32)> = vec![];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                blocks.push((i as i32, j as i32));
            }
        }
    }
    return blocks;
}

fn find_blocks2(map: &mut Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut blocks: Vec<(i32, i32)> = vec![];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '[' {
                blocks.push((i as i32, j as i32));
            }
        }
    }
    return blocks;
}

pub fn main() {
    let input = std::fs::read_to_string("inputs/day15.input").unwrap();
    let (map, moves): (&str, &str) = input.split_once("\n\n").unwrap();
    let mut map1 = map
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut robot = find_robot(&mut map1);
    moves.chars().enumerate().for_each(|(i, c)| {
        match c {
            '<' => robot = mv(&mut map1, robot.0, robot.1, 0, -1),
            '>' => robot = mv(&mut map1, robot.0, robot.1, 0, 1),
            '^' => robot = mv(&mut map1, robot.0, robot.1, -1, 0),
            'v' => robot = mv(&mut map1, robot.0, robot.1, 1, 0),
            _ => println!("error1 {}", c),
        };
    });
    let final_blocks = find_blocks(&mut map1);
    let total = final_blocks.iter().map(|(a, b)| 100 * a + b).sum::<i32>();
    println!("Part 1: {}", total);

    // Part 2
    let mut map2 = map
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    '#' => ['#', '#'],
                    '.' => ['.', '.'],
                    _ => unreachable!("error2"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    print_map(&mut map2);
    let mut robot2 = find_robot(&mut map2);
    moves.chars().enumerate().for_each(|(i, c)| {
        // println!("Step {} {:?} {}", i, robot2, c);
        match c {
            '<' => robot2 = mv2(&mut map2, robot2.0, robot2.1, 0, -1),
            '>' => robot2 = mv2(&mut map2, robot2.0, robot2.1, 0, 1),
            '^' => robot2 = mv2(&mut map2, robot2.0, robot2.1, -1, 0),
            'v' => robot2 = mv2(&mut map2, robot2.0, robot2.1, 1, 0),
            _ => println!("error3 {}", c),
        };
        // print_map(&mut map2);
        // std::io::stdin().read_line(&mut String::new()).unwrap();
    });
    let final_blocks2 = find_blocks2(&mut map2);
    let total2 = final_blocks2.iter().map(|(a, b)| 100 * a + b).sum::<i32>();
    print_map(&mut map2);
    println!("Part 2: {}", total2);
}
