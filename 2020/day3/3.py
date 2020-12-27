#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = 0
part_2 = 1


def solve(slope):
    CURRENT_X = 0
    CURRENT_Y = 0

    count = 0
    while CURRENT_Y < MAX_Y - 1:
        CURRENT_X += slope[0]
        CURRENT_Y += slope[1]
        if inp[CURRENT_Y][CURRENT_X % MAX_X] == "#":
            count += 1
    return count


# Code
MAX_X = len(inp[0])
MAX_Y = len(inp)

answers = []

slopes = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2)
]

trees = {}
for slope in slopes:
    trees[slope] = solve(slope)
    part_2 *= trees[slope]

part_1 = trees[(3, 1)]

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
