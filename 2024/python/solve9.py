from base import *

with open("input9", "r") as file:
    disk = file.read().strip()
    disk += str(0) # Append zero space padding to the end

# Load disk into list structure, e.g. [[1,1,1], 3, ...] is 3 block file with ID 1 and 3 spaces after it
fileID = -1
disk = flatten_list([(int(disk[i]) * [fileID := fileID + 1], int(disk[i+1]))for i in range(0, len(disk), 2)])

# Iterate through disk, if you find a file, append it to disk_ret, if you find space, fill it from the end of disk_list
def disk_reorganize(disk):
    disk_ret = []
    for i in disk:
        if type(i) == list:
            disk_ret.append(i)
            continue
        while i > 0:
            if type(disk[-1]) != list:
                disk.pop()
            if i > len(disk[-1]):
                i -= len(disk[-1])
                disk_ret.append(disk.pop())
            else:
                disk_ret.append(disk[-1][:i])
                disk[-1] = disk[-1][i:]
                i = 0
    return flatten_list(disk_ret) 

# Calculate checksum of a disk stored as a list of fileIDs
def checksum(disk):
    return sum([idx * i for idx, i in enumerate(disk)])

# Solution 1
print1(checksum(disk_reorganize(disk.copy())))

# Look for fittable spaces in disk for file with given size
def fittable_spaces(disk, size):
    possible_fits = [idx for idx, x in enumerate(disk) if type(x) != list and x >= size]
    return possible_fits[0] if possible_fits else []

# Move a single file in disk (most right) to the most left newest possition where it can fit, return False if no moves are possible
def file_single_move(disk):
    rev_idx = len(disk) - 1
    completed = []
    while rev_idx >= 1:
        val = disk[rev_idx]
        if type(val) != list or val in completed:
            rev_idx -= 1
            continue
        fits = fittable_spaces(disk, len(val))
        if not fits or fits >= rev_idx:
            completed.append(val)
            rev_idx -= 1
            continue
        disk.insert(fits, disk.pop(rev_idx))
        disk.insert(rev_idx+1, len(val))
        disk[fits+1] -= len(val)
        completed.append(val)
    return disk

# Solution 2
disk = flatten_list([x if type(x) == list else [0] * x for x in file_single_move(disk)])
print2(checksum(disk))