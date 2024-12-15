use std::fs::File;
use std::io::Read;

pub type Coord = (i32, i32);

pub struct Grid<T> {
    grid: Vec<Vec<T>>,
    width: i32,
    height: i32,
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
    pub fn find(&self, needle: T) -> Vec<(usize, usize)> {
        self.grid.iter().enumerate().flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &cell)| {
                if cell == needle {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .collect()
    }

    // Check if a coordinate is valid
    pub fn valid_coords(&self, c: &Coord) -> bool {
        c.0 >= 0 && c.1 >= 0 && c.0 < self.height as i32 && c.1 < self.width as i32
    }

    // Get the value at a coordinate
    pub fn at(&self, c: &Coord) -> Option<T> {
        if self.valid_coords(c) {
            Some(self.grid[c.0 as usize][c.1 as usize])
        } else {
            None
        }
    }

}

impl Grid<char> {
    pub fn from_file_as_chars(filename: &str) -> Grid<char> {
        let mut grid = Grid::new();
        grid.read_to_grid(filename, |c| c);
        grid
    }
}

impl Grid<i8> {
    pub fn from_file_as_i8(filename: &str) -> Grid<i8> {
        let mut grid = Grid::new();
        grid.read_to_grid(filename, |c| c.to_digit(10).unwrap() as i8);
        grid
    }
}

