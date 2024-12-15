import operator
from base import *

# Read file
equations = []
with open('input7') as f:
    for line in f:
        result, numbers = line.strip().split(":")
        equations.append((int(result), list(map(int, numbers.split()))))

# Recursive function to solve the equation
def solve(operators, result, numbers):
    if len(numbers) == 1:
        return numbers[0] == result
    for f in operators:
        x, y = numbers[:2]
        if solve(operators, result, [f(x,y)] + numbers[2:]):
            return True

# Concatenate two numbers
def concat(x, y):
    return int(operator.concat(str(x), str(y)))

# Solution 1 - using plus and multiply
print1(sum([eq[0] for eq in equations if solve([operator.add, operator.mul], eq[0], eq[1])]))
# Solution 2 - using plus, multiply and concatenate
print2(sum([eq[0] for eq in equations if solve([operator.add, operator.mul, concat], eq[0], eq[1])]))