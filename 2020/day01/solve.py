#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [int(line.strip()) for line in fileinput.input()]
# Variables for storing the results
part_1 = None
part_2 = None

# Code
for i in inp:
    for j in inp:
        if i+j == 2020:
            part_1 = i*j
        for k in inp:
            if i+j+k == 2020:
                part_2 = i*j*k

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
