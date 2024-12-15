import re

""" List operations """

def list_pairs(l):
    return [[x, y] for (idx, x) in enumerate(l) for y in l[idx+1:]]

def flatten_list(l):
    return [item for sublist in l for item in sublist]

def sum_middle(l):
    return sum([ll[len(ll) // 2] for ll in l])

""" Grid operations """

class Grid():
    def __init__(self, filename=None, grid=None):
        # Returns the width, height and grid of a file
        if filename is not None:
            with open(filename, "r") as file:
                grid = file.read().splitlines()
        self.w, self.h, self.grid = len(grid[0]), len(grid), grid

    def idx_to_coords(self, idx):
        # Convert an index to coordinates
        return (idx // self.h, idx % self.w)

    def in_grid(self, x, y):
        # Check if coordinates are within the grid
        return x >= 0 and x < self.h and y >= 0 and y < self.w

    def char_at(self, x, y):
        # Return the character at coordinates
        return self.grid[x][y]

    def find_instances_raw(self, pattern):
        # Find all instances of a pattern in the grid and return them as regex matches
        return re.finditer(pattern, "".join(self.grid))

    def find_instances_coords(self, pattern):
        # Find all instances of a pattern in the grid, convert them into coordinates
        return [(x.span()[0] // self.h, x.span()[0] % self.w) for x in re.finditer(pattern, "".join(self.grid))]

    def copy(self):
        # Return a copy of the grid
        return Grid(grid=self.grid.copy())

""" Printing solutions """

def print1(result):
    print("Part 1:", result)

def print2(result):
    print("Part 2:", result)