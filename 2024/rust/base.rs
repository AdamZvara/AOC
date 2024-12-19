use std::fs::File;
use std::io::Read;
use std::fmt;

pub type Coord = (i32, i32);

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {Up, Down, Left, Right}

pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub width: i32,
    pub height: i32,
}

impl fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.grid.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> Grid<T> 
where T: PartialEq + Copy {
    // Create a new grid
    pub fn new() -> Grid<T> {
        Grid {
            grid: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    // Read a file into a grid, specifying the type to store
    fn read_to_grid<F>(&mut self, filename: &str, mut parse_fn: F)
    where
        F: FnMut(char) -> T,
    {
        let mut file = File::open(&filename).unwrap();
        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();

        let mut grid = Vec::new();
        for line in text.lines() {
            let row: Vec<T> = line.chars().map(&mut parse_fn).collect();
            grid.push(row);
        }

        self.grid = grid;
        self.height = self.grid.len() as i32;
        self.width = self.grid[0].len() as i32;
    }

    // Find all occurrences of a value in the grid
    pub fn find(&self, needle: T) -> Vec<Coord> {
        self.grid.iter().enumerate().flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &cell)| {
                if cell == needle {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .collect()
    }

    // Check if a coordinate is valid
    pub fn valid_coords(&self, c: &Coord) -> bool {
        c.0 >= 0 && c.1 >= 0 && c.0 < self.height && c.1 < self.width 
    }

    // Get the value at a coordinate
    pub fn at(&self, c: &Coord) -> Option<T> {
        if self.valid_coords(c) {
            Some(self.grid[c.0 as usize][c.1 as usize])
        } else {
            None
        }
    }

    // Set the value at a coordinate
    pub fn set(&mut self, c: Coord, val: T) {
        self.grid[c.0 as usize][c.1 as usize] = val;
    }

    // Reindex value out of the grid to loop around the grid
    pub fn reindex(&self, c: Coord) -> Coord {
        if self.valid_coords(&c) {
            c
        } else {
            let mut new_c = c;
            if c.0 < 0 {
                new_c.0 = self.height + c.0;
            } else if c.0 >= self.height {
                new_c.0 = c.0 - self.height;
            }
            if c.1 < 0 {
                new_c.1 = self.width + c.1;
            } else if c.1 >= self.width {
                new_c.1 = c.1 - self.width;
            }
            new_c
        }
    }

    // Check if point is at the edge of the grid
    pub fn is_edge(&self, c: &Coord) -> bool {
        if c.0 == 0 || c.0 == self.height - 1 || c.1 == 0 || c.1 == self.width - 1  {
            true
        } else {
            false
        }
    }

    // Get coordinates of valid neighboring points (only up, down, left, right)
    pub fn neighbors(&self, c: &Coord) -> Vec<Coord> {
        let mut neigh = Vec::new();
        let directions = [(0, -1), (-1, 0), (1, 0), (0, 1)];
        for d in directions.iter() {
            let neigh_coords = (c.0 as i32 + d.0, c.1 as i32 + d.1);
            if self.valid_coords(&neigh_coords) {
                neigh.push((d.0, d.1));
            }
        }
        neigh
    }

    // Get coordinates of valid neighboring points with the same value (only up, down, left, right)
    pub fn same_neighbors(&self, c: &Coord) -> Vec<Coord> {
        let mut neigh = Vec::new();
        let val = self.at(&(c.0, c.1)).unwrap();
        for d in self.neighbors(&c) {
            let new_pos = (c.0 + d.0, c.1 + d.1);
            if self.at(&new_pos).unwrap() == val {
                neigh.push(d);
            }
        }
        neigh
    }

}

impl Grid<char> {
    // Read a file into a grid of characters
    pub fn from_file_as_chars(filename: &str) -> Grid<char> {
        let mut grid = Grid::new();
        grid.read_to_grid(filename, |c| c);
        grid
    }

    pub fn new_size(w: u32, h: u32) -> Grid<char> {
        Grid {
            grid: vec![vec!['.'; w as usize]; h as usize],
            width: w as i32,
            height: h as i32,
        }
    }
}

impl Grid<i8> {
    // Read a file into a grid of i8
    pub fn from_file_as_i8(filename: &str) -> Grid<i8> {
        let mut grid = Grid::new();
        grid.read_to_grid(filename, |c| c.to_digit(10).unwrap() as i8);
        grid
    }
}

