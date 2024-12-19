mod base;

use std::fs::File;
use std::io::read_to_string;
use base::Grid;
use base::Coord;

fn parse_input(filename: &str) -> Vec<(Coord, (i32, i32))> {
    let file = File::open(filename).expect("File not found");
    let inp = read_to_string(file).expect("Failed to read file");
    let mut result = Vec::new();
    for line in inp.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let pos = &parts[0][2..].split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let vel = &parts[1][2..].split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        // Push pos in reversed order to match the grid implementation (row, col)
        result.push(((pos[1], pos[0]), (vel[1], vel[0])));
    }
    result
}

// Check if no bots overlap
fn no_overlap(robots: &Vec<(Coord, (i32, i32))>) -> bool {
    let mut set = std::collections::HashSet::new();
    for robot in robots.iter() {
        if !set.insert(robot.0) {
            return false;
        }
    }
    true
}

// Reset grid and reposition robots
fn grid_reset(robots: &Vec<(Coord, (i32, i32))>, grid: &mut Grid<char>) {
    grid.grid.iter_mut().for_each(|row| row.iter_mut().for_each(|cell| *cell = '.'));
    for robot in robots.iter() {
        if grid.at(&robot.0).unwrap() == '.' {
            grid.set(robot.0, '1');
        } else {
            let val = grid.at(&robot.0).unwrap().to_string().parse::<i32>().unwrap();
            grid.set(robot.0, (val + 1).to_string().chars().next().unwrap());
        }
    }
}

// Simulate the robots moving
fn simulate(robots: &mut Vec<(Coord, (i32, i32))>, grid: &mut Grid<char>, steps: u32) {
    for _ in 0..steps {
        for robot in robots.iter_mut() {
            let pos = robot.0;
            let new_pos = grid.reindex((pos.0 + robot.1.0, pos.1 + robot.1.1));
            robot.0 = new_pos;
        }
    }
}

// Calculate the number of robots in each quadrant
fn quadrants(robots: &Vec<(Coord, (i32, i32))>, grid: &Grid<char>) -> (i32, i32, i32, i32) {
    let horizontal = grid.width / 2; 
    let vertical = grid.height / 2;
    let mut q = (0, 0, 0, 0);
    for robot in robots.iter() {
        if robot.0.1 < horizontal {
            if robot.0.0 < vertical {
                q.0 += 1;
            } else if robot.0.0 > vertical {
                q.1 += 1;
            }
        } else if robot.0.1 > horizontal {
            if robot.0.0 < vertical {
                q.2 += 1;
            } else if robot.0.0 > vertical {
                q.3 += 1;
            }
        }
    }
    q
}

fn main() {
    let mut robots: Vec<(Coord, (i32, i32))> = parse_input("input14");
    let mut robots_copy = robots.clone();
    let mut grid: Grid<char> = Grid::new_size(101, 103);
    
    simulate(&mut robots, &mut grid, 100);
    let q = quadrants(&robots, &grid);
    println!("Part 1: {}", q.0 * q.1 * q.2 * q.3);

    for i in 1..10000 {
        simulate(&mut robots_copy, &mut grid, 1);
        if no_overlap(&robots_copy) {
            grid_reset(&robots_copy, &mut grid);
            println!("Part 2: {}\n{}", i, grid);
        }
    }
}
