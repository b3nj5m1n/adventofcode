#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = None
part_2 = None

# Code
PREAMBLE_LENGTH = 25
numbers = []
pre_calcs = []
for i in range(0, len(inp)):
    number = int(inp[i])
    numbers.append(number)
    pre_calcs.append([])
    for j in range(min(PREAMBLE_LENGTH, len(numbers))):
        pre_calcs[i].append(number + numbers[-j])
    if i > PREAMBLE_LENGTH:
        valid = False
        for predessesor in pre_calcs[-PREAMBLE_LENGTH:]:
            if number in predessesor:
                valid = True
        if not valid:
            part_1 = number

block_len = 2
found = False
while block_len < len(inp) and not found:
    current_block = []
    current_sum = 0
    for IP in inp:
        current_block.append(int(IP))
        if len(current_block) > block_len:
            current_block.pop(0)
        current_sum = sum(current_block)
        if current_sum == part_1:
            found = True
            current_block.sort()
            part_2 = (current_block[0] + current_block[-1])
    block_len += 1

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
