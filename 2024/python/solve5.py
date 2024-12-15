from base import *

# Read file - store orders as dictionary of lists and updates as list of lists
with open("input5", "r") as file:
    orders_str, updates_str = list(map(lambda x: x.split(), file.read().split("\n\n")))
    orders = {k : [] for k in range(10, 100)}
    for order in orders_str:
        (first, second) = order.split("|")
        orders[int(first)].append(int(second))
    updates = list(map(lambda x: list(map(int, x.split(","))), updates_str))

# Check if sequence in update is valid
def update_needs_swap(update):
    pairs = [(y, x) for (x, y) in list_pairs(update)]
    for (first, second) in pairs:
        if orders[first].count(second) != 0:
            return (update.index(first), update.index(second))
    return False

# Solution 1 - return sum of middle elements for updates, where update is in correct order
print("Part 1:", sum_middle([update for update in updates if not update_needs_swap(update)]))

# Correct ordering of updates in single update
def correct_update(update):
    while update_needs_swap(update):
        if pair_to_swap := update_needs_swap(update):
            (first_idx, second_idx) = pair_to_swap
            update[first_idx], update[second_idx] = update[second_idx], update[first_idx]
    return update

# Solution 2 - return sum of middle elements for corrected updates, which were in wrong order
print("Part 2:", sum_middle([correct_update(update) for update in updates if update_needs_swap(update)]))