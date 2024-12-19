mod base;

// Define directions of movement
const DIRECTIONS: [base::Coord; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// Recursive function to find the trail
fn trail(grid: &base::Grid<i8>, pos: base::Coord, trail_ends: &mut Vec<base::Coord>) {
    let pos_value = grid.at(&pos).unwrap();

    if pos_value == 9 {
        trail_ends.push(pos);
        return;
    }

    for dir in DIRECTIONS.iter() {
        let new_pos: base::Coord = (pos.0 + dir.0, pos.1 + dir.1 );
        if grid.valid_coords(&new_pos) && grid.at(&new_pos).unwrap() == pos_value + 1 {
            trail(grid, new_pos, trail_ends);
        }
    }
}

fn main() {
    let grid = base::Grid::from_file_as_i8("input10");
    let trailheads = grid.find(0);
    let mut trail_scores = 0;
    let mut trail_ratings = 0;

    for trailhead in trailheads.iter() {
        let mut current_trail_ends: Vec<base::Coord> = Vec::new();
        trail(&grid, (trailhead.0 as i32, trailhead.1 as i32), &mut current_trail_ends);
        trail_ratings += current_trail_ends.len();
        // Sort and remove duplicates for the current trail
        current_trail_ends.sort_unstable();
        current_trail_ends.dedup();
        trail_scores += current_trail_ends.len();
    }

    println!("Part 1: {}", trail_scores);
    println!("Part 2: {}", trail_ratings);
}
