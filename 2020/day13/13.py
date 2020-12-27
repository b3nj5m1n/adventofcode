#!/usr/bin/env python3
import fileinput
import re
import math
from functools import reduce

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = None
part_2 = None

# Code
t_stamp = int(inp[0])

bus_ids = []
bus_earliest_times = {}
for i in inp[1].split(","):
    if i == "x":
        continue
    num = int(i)
    earliest_time = math.ceil((t_stamp/num))*num
    bus_ids.append(num)
    bus_earliest_times[num] = earliest_time

earliest_time = [bus_ids[0], bus_earliest_times[bus_ids[0]]]
for key, value in bus_earliest_times.items():
    if value < earliest_time[1]:
        earliest_time = [key, value]

time_to_wait = earliest_time[1] - t_stamp

part_1 = time_to_wait * earliest_time[0]

busses = []
ids = []
offsets = []
counter = 0
for i in inp[1].split(","):
    if i == "x":
        busses.append("x")
    else:
        num = int(i)
        busses.append(num)
        ids.append(num)
        offsets.append(counter)
    counter += 1

last_time = ids[0]
adder = ids[0]
for i in range(1, len(ids)):
    previous = ids[i-1]
    current = ids[i]
    delay = offsets[i]
    time = last_time
    done = False
    while not done:
        time += adder
        valid = True
        for j in range(len(ids[:1+i])):
            if not (time + offsets[j]) % ids[j] == 0:
                valid = False
        done = valid
    adder = reduce(lambda x, y: x*y, ids[:1+i])
    last_time = time
    part_2 = last_time

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
