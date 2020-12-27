#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = 0
part_2 = 0


def seat_row(inp):
    ROW = inp[:7]
    rows = list(range(0, 127+1))
    for char in ROW:
        if char == "F":
            rows = rows[:len(rows)//2]
        elif char == "B":
            rows = rows[len(rows)//2:]
    seat_row = rows[0]
    return seat_row


def seat_column(inp):
    COLUMN = inp[-3:]
    columns = range(0, 7+1)
    for char in COLUMN:
        if char == "L":
            columns = columns[:len(columns)//2]
        elif char == "R":
            columns = columns[len(columns)//2:]
    seat_column = columns[0]
    return seat_column


def seat_id(inp):
    seat_id = seat_row(inp) * 8 + seat_column(inp)
    return seat_id


# Code
ids = []
for seat in inp:
    id = seat_id(seat)
    ids.append(id)
    if id > part_1:
        part_1 = id

ids.sort()
for i in range(0, len(ids)):
    if ids[i]+1 not in ids and ids[i]+2 in ids:
        part_2 = ids[i]+1

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
