from base import *

# Read the input file
grid = Grid("input8")
    
# Find all antennas
antennas = {a.group(): [(grid.idx_to_coords(match.span()[0])) for match in grid.find_instances_raw("[^.]") if match.group() == a.group()] for a in grid.find_instances_raw("[^.]")}

# Return antinodes of two antennas in double the distance
def antinode_distance(x, y, vector = None):
    vec = [x[0] - y[0], x[1] - y[1]] if vector is None else vector
    return [tuple(a) for a in [[x[0] + vec[0], x[1] + vec[1]], [y[0] - vec[0], y[1] - vec[1]]]]

# Solution 1
antinodes1 = flatten_list([antinode_distance(*p) for a in antennas for p in list_pairs(antennas[a])])
print1(len(set([x for x in antinodes1 if grid.in_grid(*x)])))

# Return antinodes of two antennas in bigger distances
def antinode_all(x, y):
    vec = [x[0] - y[0], x[1] - y[1]]
    result = []
    while any([grid.in_grid(*a) for a in [x, y]]):
        x, y = antinode_distance(x, y, vec)
        if grid.in_grid(*x):
            result.append(x)
        if grid.in_grid(*y):
            result.append(y)
    return result

# Solution 2
antinodes2 = flatten_list([antinode_all(*p) for a in antennas for p in list_pairs(antennas[a])]) + flatten_list([antennas[a] for a in antennas if len(antennas[a]) > 1])
print2(len(set(antinodes2)))