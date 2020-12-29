#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = None
part_2 = None

# Code
[print(line) for line in inp]

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
