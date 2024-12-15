import re, math
from base import *

# Read the input file
with open("input3", "r") as file:
    prog = file.read()

# Calculate the multiplication of two values from expression
def calculate_mul(expr):
    return math.prod(map(int, expr[4:-1].split(",")))

# Solution 1 - sum all valid multiplication expressions
print1(sum([calculate_mul(mul) for mul in re.findall(r"mul\(\d+,\d+\)", prog)]))

# Find the closest number in a list that is lower than x
def closest_lower_number(x, numbers):
    return max(lower_nums) if (lower_nums := [n for n in numbers if n < x]) else -1e20

# Solution 2 - find indexes of all "do()" and "don't()" expressions and for each valid multiplication expression
# check if the closest "do()" is closer than the closest "don't()"
muls = re.finditer(r"mul\(\d+,\d+\)", prog)
dos = [0] + [x.span()[0] for x in re.finditer(r"do\(\)", prog)]
donts = [x.span()[0] for x in re.finditer(r"don\'t\(\)", prog)]

print2(sum([calculate_mul(mul.group()) for mul in muls if closest_lower_number(mul.span()[0], dos) > closest_lower_number(mul.span()[0], donts)]))