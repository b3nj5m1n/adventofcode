#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = None
part_2 = None


def surrounding(coordinates, part):
    x, y, z, v = coordinates
    result = []
    for i in [-1, 0, 1]:
        for j in [-1, 0, 1]:
            for k in [-1, 0, 1]:
                for t in [-1, 0, 1] if part == 2 else [0]:
                    if not i == j == k == t == 0:
                        result.append((x + i, y + j, z + k, v + t))
    return result


def solve(grid, size, part):
    for i in range(6):
        tmpg = grid.copy()
        for coordinates in [
            (x, y, z, v)
            for x in range(-size, size + 1)
            for y in range(-size, size + 1)
            for z in range(-size, size + 1)
            for v in range(-size, size + 1)
        ]:
            state = 0
            if not coordinates in tmpg:
                grid[coordinates] = 0
            else:
                state = tmpg[coordinates]
            active_count = 0
            for neigbour in surrounding(coordinates, part):
                if neigbour in tmpg:
                    active_count += tmpg[neigbour]
            if state == True and not active_count in [2, 3]:
                grid[coordinates] = 0
            elif state == False and active_count == 3:
                grid[coordinates] = 1
        size += 1
    return sum(grid.values())


# Code
grid = {}
for i, x in enumerate(inp):
    for j, y in enumerate(x):
        grid[(i, j, 0, 0)] = 1 if y == "#" else 0
size = len(inp[0])

part_1 = solve(grid.copy(), size, 1)
part_2 = solve(grid.copy(), size, 2)

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
