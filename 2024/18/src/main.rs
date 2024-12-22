mod base;

use std::collections::HashMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use base::Grid;
use base::Coord;

// Read falling bytes from file into vector of coordinates
fn read_input(filename: &str) -> Vec<Coord> {
    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut result = Vec::new();
    for line in contents.lines() {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<i32>().unwrap();
        let y = parts.next().unwrap().parse::<i32>().unwrap();
        result.push((y, x));
    }
    result
}

// Simulate n steps of falling bytes, return last position
fn n_steps(grid: &mut Grid<char>, memory: &mut Vec<Coord>, n: u32) -> Coord {
    let mut pos = (0, 0);
    for _ in 0..n {
        pos = memory.pop().unwrap();
        grid.set(pos, '#');
    }
    pos
}

// Dijsktra's algorithm to find shortest path from start to end
fn dijkstra(grid: &Grid<char>, start: Coord, end: Coord) -> HashMap<Coord, i32> {
    let mut pq = PriorityQueue::new();
    let mut dist = std::collections::HashMap::new();
    let mut visited = std::collections::HashSet::new();

    pq.push(start, Reverse(0));
    dist.insert(start, 0);

    while let Some((current, _)) = pq.pop() {
        if current == end {
            return dist;
        }

        visited.insert(current);

        for neighbor in grid.same_neighbors(&current) {
            let neigh_coords = (current.0 + neighbor.0, current.1 + neighbor.1);
            if visited.contains(&neigh_coords) {
                continue;
            }

            let new_dist = dist[&current] + 1;
            if !dist.contains_key(&neigh_coords) || new_dist < dist[&neigh_coords] {
                dist.insert(neigh_coords, new_dist);
                pq.push(neigh_coords, Reverse(new_dist));
            }
        }
    }

    dist
}

fn main() {
    let width = 71;
    let filename = "input";
    let height = width;
    let mut grid = Grid::new_size(width, height); 
    let mut coords = read_input(filename);

    // Reverse coords to make popping easier
    coords.reverse();
    let first_steps = 1024;
    n_steps(&mut grid, &mut coords, first_steps);

    // Get the shortest path
    let start = (0, 0);
    let end = (grid.height - 1, grid.width - 1);
    let dist = dijkstra(&grid, start, end);
    println!("Part 1: {:?}", dist.get(&end).unwrap());

    // Try to add a step at a time and check if end is still reachable
    for _ in 0..coords.len() {
        let pos = n_steps(&mut grid, &mut coords, 1);
        let dist = dijkstra(&grid, start, end);
        if dist.get(&end).is_none() {
            // Need to reverse position as I use different indexing in grid
            println!("Part 2: {:?}", (pos.1, pos.0));
            break;
        }
    }
}
