from base import *

# Read grid
grid = Grid("input4")
    
# Find positions of X letter in the grid
X = grid.find_instances_coords("X")

# Generate all possible directions with valid coordinates (not out of the grid)
def all_8_directions(grid, x, y):
    up_down = list(zip(*[((x + i, y), (x - i, y), (x, y + i), (x, y - i)) for i in range(4)]))
    diagonal = list(zip(*[((x + i, y + i), (x - i, y - i), (x - i, y + i), (x + i, y - i)) for i in range(4)]))
    return [d for d in up_down + diagonal if all(grid.in_grid(*p) for p in d)]

# Solution 1 - Count the number of "XMAS" in all directions
print1(["".join([grid.char_at(*p) for p in i]) for x in X for i in all_8_directions(grid, *x)].count("XMAS"))

# Find positions of A letter (middle letter) 
A = grid.find_instances_coords("A")

# Generate all possible diagonal directions with valid coordinates (not out of the grid) for 3 characters
def diagonal_directions(grid, x, y):
    diagonal = [(x-1, y-1), (x+1, y+1), (x+1, y-1), (x-1, y+1)]
    return diagonal if all(grid.in_grid(*p) for p in diagonal) else []

# Solution 2
words = ["".join([grid.char_at(*p) for p in diagonal_directions(grid, *x)]) for x in A]
print2(sum([words.count(target) for target in ["MSMS", "MSSM", "SMMS", "SMSM"]]))