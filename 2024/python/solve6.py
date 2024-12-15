from base import *

# Read the input grid
grid = Grid("input6")
guard = grid.find_instances_coords(r"\^")[0]

# Define the directions of movement (up, right, down, left)
directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]

# Run the guard through the grid until it hits an obstacle, if cycle is detected return None
def guard_run(grid, guard):
    direction = 0
    path = [(guard[0], guard[1])]
    directions_visited = {(guard[0], guard[1]) : direction} 
    while True:
        new_pos = (guard[0] + directions[direction][0], guard[1] + directions[direction][1])
        if not grid.in_grid(*new_pos):
            break
        if new_pos in directions_visited and directions_visited[new_pos] == direction: # cycle detected
            return None
        if grid.char_at(*new_pos) == "#":
            direction = (direction + 1) % 4
            continue
        path.append(new_pos)
        directions_visited[new_pos] = direction
        guard = new_pos
    return path
        
# Solution 1
guard_path = guard_run(grid, guard)
print1(len(set(guard_path)))

# Generate new grids with obstacles from the previous path
def new_obstacle_grids(grid, path):
    for (x, y) in path:
        grid_cpy = grid.copy()
        grid_cpy.grid[x] = grid_cpy.grid[x][:y] + "#" + grid_cpy.grid[x][y + 1:]
        yield grid_cpy

# Solution 2
print2(len([new_grid for new_grid in new_obstacle_grids(grid, set(guard_path[1:])) if guard_run(new_grid, guard) is None]))