mod base;

use base::Grid;
use base::Coord;
use base::Direction;

use std::collections::HashMap;

// Seed fill algorithm to find connected regions of the same plant
fn seed_fill(grid: &Grid<char>, start: &Coord) -> Vec<Coord> {
    let mut stack = Vec::new();
    let mut visited = Vec::new();
    stack.push(*start);

    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        visited.push(current);

        for (x, y) in grid.same_neighbors(&current) {
            let new_pos = (current.0 + x, current.1 + y);
            if !visited.contains(&new_pos) {
                stack.push(new_pos);
            }
        }
    }

    visited
}

// Form connected regions of the same plant within the grid from vector of plant positions
fn connected_regions(grid: &Grid<char>, plants: &Vec<Coord>) -> Vec<Vec<Coord>> {
    let mut connected_regions = Vec::new();
    let mut visited = Vec::new();
    for r in plants {
        if !visited.contains(r) {
            let region = seed_fill(grid, r);
            visited.extend(region.clone());
            connected_regions.push(region);
        }
    }
    connected_regions
}

// A simple perimeter is given by the number of neighbors of a plant that are not the same plant
fn simple_perimeter(grid: &Grid<char>, reg: &Vec<Coord>) -> u32 {
    let mut perimeter: u32 = 0;
    for r in reg {
        let neigh = grid.same_neighbors(r).len();
        if neigh < 4 {
            perimeter += 4 - neigh as u32;
        }
    }
    perimeter * reg.len() as u32
}

// Return vector of directions for fences at a given position
fn fences_at(grid: &Grid<char>, pos: &Coord) -> Vec<Direction> {
    let mut fences = Vec::new();
    let val = grid.at(&(pos.0, pos.1)).unwrap();
    // Append fences for each neigbor different from the current plant
    for n in grid.neighbors(&pos) {
        let new_pos = (pos.0 + n.0, pos.1 + n.1);
        if grid.at(&new_pos).unwrap() != val {
            let direction = match n {
                ( 0,-1) => Direction::Left,
                (-1, 0) => Direction::Up,
                ( 1, 0) => Direction::Down,
                _       => Direction::Right,
            };
            fences.push(direction);
        } 
    }
    // Edges
    if grid.is_edge(pos) {
        if pos.0 == 0 {
            fences.push(Direction::Up);
        }
        if pos.0 == grid.height - 1 {
            fences.push(Direction::Down);
        }
        if pos.1 == 0 {
            fences.push(Direction::Left);
        }
        if pos.1 == grid.width - 1 {
            fences.push(Direction::Right);
        }
    }
    fences
}

// Return a hashmap of fences for each region
fn region_fences(grid: &Grid<char>, reg: &Vec<Coord>) -> HashMap<Coord, Vec<Direction>> {
    let mut fences = HashMap::new();
    for r in reg {
        fences.insert(r.clone(), fences_at(grid,r));
    }
    fences
}

/* Calculate outer corners for given directions and diagonal neighbors
   We need to handle cases like:
   A  A  A
   A  X  A
   A |A| X
   A  X  A
   Where the |A| plant is NOT an outer corner as it has a neighbors in the diagonal direction
*/
fn outer_corner(directions: &Vec<Direction>, diag_neigh: &[Option<&Vec<Direction>>], corners: &mut u32) {
    if directions.contains(&&Direction::Down) && directions.contains(&&Direction::Right) && diag_neigh[1].is_none() {
        *corners += 1;
    } 
    if directions.contains(&&Direction::Down) && directions.contains(&&Direction::Left) && diag_neigh[3].is_none() {
        *corners += 1;
    }
    if directions.contains(&&Direction::Up) && directions.contains(&&Direction::Left) && diag_neigh[2].is_none() {
        *corners += 1;
    }
    if directions.contains(&&Direction::Up) && directions.contains(&&Direction::Right) && diag_neigh[0].is_none() {
        *corners += 1;
    }
}

// Calculate corners for a given region
fn corners(fences: &HashMap<Coord, Vec<Direction>>) -> u32 {
    let mut corners = 0;
    for (coord, directions) in fences {
        let diag_neighbors = [
            fences.get(&(coord.0 - 1, coord.1 + 1)), // up right
            fences.get(&(coord.0 + 1, coord.1 + 1)), // down right
            fences.get(&(coord.0 - 1, coord.1 - 1)), // up left
            fences.get(&(coord.0 + 1, coord.1 - 1))  // down left
        ]; 
        // Outer corners
        match directions.len() {
            4 => { // 4 fences = 4 corners
                corners += 4 
            }, 
            3 => { // Only if not interfering with inner corners
                outer_corner(directions, &diag_neighbors, &mut corners) 
            }, 
            2 => { // Only if not interfering with inner corners and not parallel
                if !(directions.contains(&&Direction::Up) && directions.contains(&&Direction::Down)) && 
                   !(directions.contains(&&Direction::Left) && directions.contains(&&Direction::Right)) {
                        outer_corner(directions, &diag_neighbors, &mut corners);
                   }
                },
            _ => { 
                corners += 0 // 1 or 0 fences = 0 corners
            } 
        };
        // Inner corners
        if directions.contains(&&Direction::Right) {
            if diag_neighbors[0].is_some() && diag_neighbors[0].unwrap().contains(&Direction::Down) {
                corners += 1;
            }
            if diag_neighbors[1].is_some() && diag_neighbors[1].unwrap().contains(&Direction::Up) {
                corners += 1;
            }
        }
        if directions.contains(&&Direction::Left) {
            if diag_neighbors[2].is_some() && diag_neighbors[2].unwrap().contains(&Direction::Down) {
                corners += 1;
            }
            if diag_neighbors[3].is_some() && diag_neighbors[3].unwrap().contains(&Direction::Up) {
                corners += 1;
            }
        }
    }
    corners
}

fn main() {
    let grid = Grid::from_file_as_chars("input");
    let mut unique_plants: Vec<char> = grid.grid.iter().flatten().map(|x| *x).collect();
    unique_plants.sort();
    unique_plants.dedup();

    let mut cost1 = 0;
    let mut cost2 = 0;
    for plant in unique_plants.iter() {
        // Find positions of the same plant within the grid
        let plants = &grid.find(*plant);

        // Form vector of plants into connected regions
        let connected_regions = connected_regions(&grid, plants);
        for cr in connected_regions {
            // For the first part, calculate the perimeter of the region
            cost1 += simple_perimeter(&grid, &cr);
            let fences = region_fences(&grid, &cr);
            cost2 += corners(&fences) * cr.len() as u32;
        }
    }

    println!("Part 1: {}", cost1);
    println!("Part 2: {}", cost2);
}