mod base;

use std::cmp::Reverse;
use std::collections::HashMap;
use base::Grid;
use base::Coord;
use priority_queue::PriorityQueue;

fn dijkstra(grid: &Grid<char>, start: &Coord, end: &Coord) -> HashMap<Coord, i32> {
    let mut pq = PriorityQueue::new();
    let mut dist = HashMap::new();

    pq.push(*start, Reverse(0));
    dist.insert(*start, 0);

    while let Some((current, _)) = pq.pop() {
        if current == *end {
            break;
        }

        for neighbor in grid.same_neighbors(&current) {
            let new_pos = (current.0 + neighbor.0, current.1 + neighbor.1);
            let new_dist = dist[&current] + 1;
            if !dist.contains_key(&new_pos) || new_dist < dist[&new_pos] {
                dist.insert(new_pos, new_dist);
                pq.push(new_pos, Reverse(new_dist));
            }
        }
    }

    dist
}

fn mann_dist(a: &Coord, b: &Coord) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

// Get all neighbors reachable from a point (start) in a certain distance
// This function is recursive and starts with start == current point
fn get_neig_in_dist(grid: &Grid<char>, start: &Coord, current: &Coord, neighbors: &mut Vec<Coord>, visited: &mut Vec<Coord>, dist: u32) {
    if visited.contains(current) {
        return;
    }

    visited.push(*current);

    for n in grid.neighbors(current) {
        let new_neigh = (current.0 + n.0, current.1 + n.1);
        if visited.contains(&new_neigh) {
            continue;
        }
        if mann_dist(start, &new_neigh) <= dist {
            if grid.at(&new_neigh).unwrap() == '.' {
                neighbors.push(new_neigh);
            }
            get_neig_in_dist(grid, start, &new_neigh, neighbors, visited, dist);
        }
    }
}

// See how many cheats > 100 are possible for various cheat steps
fn solve(paths: &Vec<Coord>, grid: &Grid<char>, dist: &HashMap<Coord, i32>, original_distance: i32, cheat_steps: u32) -> u32 {
    let mut cheats_map = HashMap::new();

    let mut neighbors = Vec::new();
    let mut visited = Vec::new();

    // For each point in path
    for p in paths {
        neighbors.clear();
        visited.clear();
        // Look for all possible cheats (neighbors with distance < cheat_steps)
        get_neig_in_dist(&grid, &p, &p, &mut neighbors, &mut visited, cheat_steps);
        for n in &neighbors {
            if dist[&p] > dist[&n] { // If the neighbor is going back to the start
                continue;
            }
            // Calculate the cutted distance
            let cutted = original_distance - (dist[&p] + mann_dist(&p, &n) as i32 + (original_distance - dist[&n]));
            if cheats_map.contains_key(&cutted) {
                cheats_map.insert(cutted, cheats_map[&cutted] + 1);
            } else {
                cheats_map.insert(cutted, 1);
            }
        }
    }

    let mut sum = 0;
    for (k, v) in &cheats_map {
        if *k >= 100 {
            sum += v;
        }
    }

    sum
}

fn main() {
    let mut grid = Grid::from_file_as_chars("input"); 

    // Get the shortest path
    let start = grid.find('S')[0];
    let end = grid.find('E')[0];
    grid.set(start, '.');
    grid.set(end, '.');

    let dist = dijkstra(&grid, &start, &end);
    let original_distance = dist[&end];
    
    let paths = grid.find('.');
    let part1 = solve(&paths, &grid, &dist, original_distance, 2);
    println!("Part 1: {}", part1);
    
    let part2 = solve(&paths, &grid, &dist, original_distance, 20);
    println!("Part 2: {}", part2);
}
