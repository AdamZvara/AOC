from base import *

# Read input data
with open("input1", "r") as file:
    # Convert each line into a tuple of integers and unzip it into two lists
    list1, list2 = zip(*[map(int, line.split()) for line in file])

# Solution 1
print1(sum([abs(v2 - v1) for v1, v2 in zip(sorted(list1), sorted(list2))]))

# Solution 2 - count occurrences of each value in list2 and multiply it by the corresponding value in list1
print2(sum([list2.count(v) * v for v in list1]))