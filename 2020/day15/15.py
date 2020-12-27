#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip().split(",") for line in fileinput.input()][0]
# Variables for storing the results
part_1 = None
part_2 = None

# Code
call_stack = []
indicies = {}
length = len(inp)

count = 0

while count < 30000000:
    num = 0
    if count < length:
        num = int(inp[count])
    else:
        occs = indicies[call_stack[-1]]
        if len(occs) == 1:
            num = 0
        else:
            num = occs[-1] - occs[-2]
    call_stack.append(num)
    if num not in indicies:
        indicies[num] = [count]
    else:
        indicies[num].append(count)
    count += 1
    if count == 2020:
        part_1 = call_stack[-1]
part_2 = call_stack[-1]

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
