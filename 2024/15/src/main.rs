mod base;

use std::fs::File;
use std::io::Read;
use std::fmt;
use std::fmt::Display;
use base::Grid;
use base::Direction;
use base::Coord;

struct GameState {
    robot: Coord,
    walls: Vec<Coord>,
    boxes: Vec<Coord>,
    w: u32, 
    h: u32
}

impl Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid = Grid::new_size(self.w, self.h);
        for wall in self.walls.iter() {
            grid.set(*wall, '#');
        }
        for b in self.boxes.iter() {
            grid.set(*b, 'O');
        }
        grid.set(self.robot, '@');
        write!(f, "{}", grid)?;
        Ok(())
    }
}

struct GameState2 {
    robot: Coord,
    walls: Vec<Coord>,
    boxes: Vec<(Coord, Coord)>,
    w: u32, 
    h: u32
}

impl Display for GameState2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid = Grid::new_size(self.w, self.h);
        for wall in self.walls.iter() {
            grid.set(*wall, '#');
        }
        for b in self.boxes.iter() {
            grid.set(b.0, '[');
            grid.set(b.1, ']');
        }
        grid.set(self.robot, '@');
        write!(f, "{}", grid)?;
        Ok(())
    }
}

impl GameState2 {
    fn contains_box(&self, pos: Coord) -> Option<(Coord, Coord)> {
        for b in self.boxes.iter() {
            if b.0 == pos || b.1 == pos {
                return Some(*b);
            }
        }
        None
    }
}

// Parse file into grid and list of moves
fn parse_input(filename: &str) -> (Grid<char>, Vec<Direction>) {
    let mut file = File::open(&filename).expect("File not found");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Failed to read file");
    let parts = text.split("\n\n").collect::<Vec<&str>>();
    let grid = Grid::from_lines(parts[0].lines().collect());
    let mut moves = Vec::new();
    for line in parts[1].lines() {
        for c in line.chars() {
            moves.push(match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("Invalid move"),
            });
        }
    }
    (grid, moves)
}

// Robot wants to move in direction but there are (one or more) boxes in the way, 
// check if you can move them and if so, move the boxes and return true, otherwise return false
fn move_boxes(game: &mut GameState, pos: Coord, direction: &Direction) -> bool {
    let mut box_sequence = vec![pos];
    let dir = match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };
    loop {
        let new_pos = (box_sequence.last().unwrap().0 + dir.0, box_sequence.last().unwrap().1 + dir.1);
        if game.walls.contains(&new_pos) {
            return false;
        } else if game.boxes.contains(&new_pos) {
            box_sequence.push(new_pos);
        } else {
            // Move the first box in the sequence to the new position
            game.boxes.retain(|&x| x != box_sequence[0]);
            game.boxes.push(new_pos);
            return true;
        }
    }
}

// Get the next position in the grid in the given direction
fn next_pos(pos: Coord, direction: &Direction) -> Coord {
    match direction {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0, pos.1 + 1),
    }
}

// Move robot within the grid in the given direction
fn robot_move(game: &mut GameState, direction: &Direction) {
    let new_pos = next_pos(game.robot, direction);
    
    if game.walls.contains(&new_pos) {
        return;
    }
    
    if game.boxes.contains(&new_pos) {
        if move_boxes(game, new_pos, direction) {
            game.robot = new_pos;
        }
        return;
    } else {
        game.robot = new_pos;
    }
}

// Find all boxes connected to the start box in the given vertical direction, if no boxes can be moved return None
fn connected_boxes(game: &GameState2, start_box: (Coord, Coord), direction: &Direction) -> Option<Vec<(Coord, Coord)>> {
    let mut boxes = vec![start_box];
    let mut connected_boxes = Vec::new();
    let movement = if *direction == Direction::Up { -1 } else { 1 };

    while !boxes.is_empty() {
        let b = boxes[0];
        let left_pos = (b.0.0 + movement, b.0.1);
        let right_pos = (b.0.0 + movement, b.0.1 + 1);
        if game.walls.contains(&left_pos) || game.walls.contains(&right_pos) {
            return None;
        }
        if game.contains_box(left_pos).is_some() {
            boxes.push(game.contains_box(left_pos).unwrap());
        }
        if game.contains_box(right_pos).is_some() {
            boxes.push(game.contains_box(right_pos).unwrap());
        }
        boxes.retain(|&x| x != b);
        connected_boxes.push(b);
    }
    
    Some(connected_boxes)
} 

// Move boxes in the wide grid in the given direction
fn move_boxes_wide(game: &mut GameState2, start_box: (Coord, Coord), direction: &Direction) -> bool {
    // Handle horizontal movement
    if *direction == Direction::Left || *direction == Direction::Right {
        let mut box_sequence = vec![start_box];
        let next_field = if *direction == Direction::Left { -1 } else { 2 };
        loop {
            // Get the next possible field
            let new_pos = (box_sequence.last().unwrap().0.0, box_sequence.last().unwrap().0.1 + next_field);
            if game.walls.contains(&new_pos) {
                return false;
            } else if game.contains_box(new_pos).is_some() {
                box_sequence.push(game.contains_box(new_pos).unwrap());
            } else {
                // Move boxes to the new position
                for b in box_sequence.iter() {
                    game.boxes.retain(|&x| x != *b);
                    // Since box_sequence contains right or left part of the box, we need to define movement for each direction
                    let box_offset = if *direction == Direction::Left { -1 } else { 1 };
                    game.boxes.push(((b.0.0, b.0.1 + box_offset), (b.1.0, b.1.1 + box_offset)));
                }
                return true;
            }
        }
    }

    // Handle vertical movement
    let connected_boxes = connected_boxes(game, start_box, direction).unwrap_or(vec![]);

    if connected_boxes.is_empty() {
        return false;
    }

    // Remove old boxes - this can't be combined with moving the boxes as new boxes might get deleted
    for b in &connected_boxes {
        game.boxes.retain(|&x| x != *b);
    }

    for b in &connected_boxes {
        let box_offset = if *direction == Direction::Up { -1 } else { 1 };
        game.boxes.push(((b.0.0 + box_offset, b.0.1), (b.1.0 + box_offset, b.1.1)));
    }

    true
}

// Move robot within the wide grid in the given direction
fn robot_move_wide(game: &mut GameState2, direction: &Direction) {
    let new_pos = next_pos(game.robot, direction);
    
    if game.walls.contains(&new_pos) {
        return;
    }
    
    let possible_box = game.contains_box(new_pos);
    if possible_box.is_some() {
        if move_boxes_wide(game, possible_box.unwrap(), direction) {
            game.robot = new_pos;
        }
        return;
    } else {
        game.robot = new_pos;
    }
}

// Print GPS sum of boxes coordinates
fn gpsum(boxes: &Vec<Coord>) -> u32 {
    let mut sum = 0;
    for b in boxes.iter() {
        sum += (b.0 * 100 + b.1) as u32;
    }
    sum
}

fn gpsum2(boxes: &Vec<(Coord, Coord)>) -> u32 {
    let mut sum = 0;
    for b in boxes.iter() {
        sum += (b.0.0 * 100 + b.0.1) as u32;
    }
    sum
}

// Widen the grid by duplicating each cell horizontally
fn grid_widen(grid: &Grid<char>) -> Grid<char> {
    let mut new_grid = Grid::new_size((grid.width * 2) as u32, grid.height as u32);
    for (row_idx, row) in grid.grid.iter().enumerate() {
        for (cell_idx, cell) in row.iter().enumerate() {
            let c = (row_idx as i32, (cell_idx * 2) as i32);
            let c2 = (row_idx as i32, (cell_idx * 2 + 1) as i32);
            match cell {
                '#' => {
                    new_grid.set(c, *cell);
                    new_grid.set(c2, *cell);
                },
                'O' => {
                    new_grid.set(c, '[');
                    new_grid.set(c2, ']');
                },
                '@' => {
                    new_grid.set(c, *cell);
                    new_grid.set(c2, '.');
                },
                _ => (),
            }
        }
    }
    new_grid
}

fn main() {
    let (grid, moves) = parse_input("input");
    let mut game_state = GameState {
        robot: grid.find('@')[0],
        walls: grid.find('#'),
        boxes: grid.find('O'),
        w: grid.width as u32,
        h: grid.height as u32
    };

    for m in moves.iter() {
        robot_move(&mut game_state, m);
        // println!("{}", game_state);
    }

    println!("Part 1: {}", gpsum(&game_state.boxes));

    let grid2 = grid_widen(&grid);
    let mut game_state2 = GameState2 {
        robot: grid2.find('@')[0],
        walls: grid2.find('#'),
        boxes: grid2.find('[').iter().map(|&x| (x, (x.0, x.1+1))).collect(),
        w: grid2.width as u32,
        h: grid2.height as u32
    }; 

    
    for m in moves.iter() {
        robot_move_wide(&mut game_state2, m);
        // println!("{}", game_state2);
    }

    println!("Part 2: {}", gpsum2(&game_state2.boxes));
}