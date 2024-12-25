use std::collections::HashSet;
use std::fs;

fn start_pos(maze: &Vec<Vec<char>>) -> (usize, usize) {
    for (x, line) in maze.iter().enumerate() {
        for (y, &c) in line.iter().enumerate() {
            if c == '^' {
                return (x, y);
            }
        }
    }
    panic!("No start position found");
}

fn print_maze(maze: &Vec<Vec<char>>) {
    for line in maze {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn move1(
    maze: &mut Vec<Vec<char>>,
    pos: (usize, usize),
    dir: (isize, isize),
) -> ((usize, usize), (isize, isize), bool) {
    // print_maze(maze);
    let (dx, dy) = dir;
    let (nx, ny) = (pos.0 as isize + dx, pos.1 as isize + dy);
    if nx < 0 || ny < 0 || nx >= maze.len() as isize || ny >= maze[nx as usize].len() as isize {
        maze[pos.0][pos.1] = 'X';
        return (pos, dir, true);
    }
    let (nx, ny) = (nx as usize, ny as usize);
    match maze[nx][ny] {
        // move forward
        '.' => {
            maze[pos.0][pos.1] = 'X';
            maze[nx][ny] = '^';
            return ((nx, ny), dir, false);
        }
        'X' => {
            maze[pos.0][pos.1] = 'X';
            maze[nx][ny] = '^';
            return ((nx, ny), dir, false);
        }
        '#' => {
            // turn right
            let new_dir = match (dx, dy) {
                (-1, 0) => (0, 1),  // ^ -> >
                (0, 1) => (1, 0),   // > -> v
                (1, 0) => (0, -1),  // v -> <
                (0, -1) => (-1, 0), // < -> ^
                _ => panic!("Invalid direction"),
            };
            return (pos, new_dir, false);
        }
        _ => panic!("Invalid character"),
    }
}

fn count(maze: &Vec<Vec<char>>) -> usize {
    maze.iter()
        .map(|line| line.iter().filter(|&&c| c == 'X').count())
        .sum()
}

pub fn main() {
    let input = fs::read_to_string("inputs/day6.input").unwrap();
    let maze: Vec<Vec<char>> = input.lines().map(|a| a.chars().collect()).collect();

    // // part 1
    // let mut maze1 = maze.clone();
    // let mut pos = start_pos(&maze1);
    // let mut dir = (-1, 0);
    // loop {
    //     let (new_pos, new_dir, done) = move1(&mut maze1, pos, dir);
    //     pos = new_pos;
    //     dir = new_dir;
    //     // println!("{} {} {:?}", pos.0, pos.1, dir);
    //     if done {
    //         break;
    //     }
    // }
    // println!("{}", count(&maze1));

    // part 2

    let mut count = 0;
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            if maze[i][j] == '#' || maze[i][j] == '^' {
                continue;
            }
            println!("{} {}", i, j);
            // read user input
            // let mut s = String::new();
            // std::io::stdin().read_line(&mut s);

            // replace with '#'
            let mut new_maze = maze.clone();
            new_maze[i][j] = '#';

            let mut pos_dir_set: HashSet<((usize, usize), (isize, isize))> = HashSet::new();
            let mut pos = start_pos(&new_maze);
            let mut dir = (-1, 0);

            let mut is_loop = false;
            loop {
                let (new_pos, new_dir, done) = move1(&mut new_maze, pos, dir);
                pos = new_pos;
                dir = new_dir;
                if done {
                    break;
                }
                if !pos_dir_set.insert((pos, dir)) {
                    is_loop = true;
                    break;
                }
            }
            if is_loop {
                count += 1;
            }
        }
    }
    println!("Part 2: {}", count);
}
