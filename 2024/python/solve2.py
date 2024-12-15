from base import *

# Read lists from file
with open("input2", "r") as file:
    data = [list(map(int, line.split())) for line in file]

# Solve single list problem - check if all distances between consecutive IDs are in range and list is monotonic
def solve_list(xlist):
    distances = [y - x for (x, y) in zip(xlist[:-1], xlist[1:])]
    return (all(x > 0 for x in distances) or all(x < 0 for x in distances)) and all(x in range(1,4) for x in [abs(x) for x in distances])

# Solution 1
print1([solve_list(l) for l in data].count(True))

# Brute force all possible lists with one element removed
def bruteforce_single_list(xlist):
    return any(solve_list(x) for x in [xlist] + [xlist[:i] + xlist[i+1:] for i in range(len(xlist))])

# Solution 2
print2([bruteforce_single_list(l) for l in data].count(True))