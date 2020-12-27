#!/usr/bin/env python3
import fileinput
from copy import deepcopy
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = None
part_2 = None


def is_in_bounds(current_layout, x, y):
    if 0 <= x < len(current_layout):
        if 0 <= y < len(current_layout[x]):
            return True
    return False


def get_surrounding_part_1(current_layout, x, y):
    results = []
    for i in range(x-1, x+2):
        for j in range(y-1, y+2):
            if is_in_bounds(current_layout, i, j) and not [i, j] == [x, y]:
                results.append([i, j])
    return results


def get_surrounding_part_2(current_layout, x, y):
    layout_height = len(current_layout)
    layout_width = len(current_layout[0])
    results = []
    for i in [-1, 0, 1]:
        for j in [-1, 0, 1]:
            if not (i == 0 and j == 0):
                rr = x+i
                cc = y+j
                while 0 <= rr < layout_height and 0 <= cc < layout_width and current_layout[rr][cc] == '.':
                    rr = rr+i
                    cc = cc+j
                if 0 <= rr < layout_height and 0 <= cc < layout_width and current_layout[rr][cc] == '#':
                    results.append([rr, cc])
    return results


def seat_new_state_part_1(current_layout, x ,y):
    neighbours = get_surrounding_part_1(current_layout, x, y)
    occupied_neighbours = 0
    unoccupied_neighbours = 0
    for i, j in neighbours:
        if current_layout[i][j] == "L":
            unoccupied_neighbours += 1
        elif current_layout[i][j] == "#":
            occupied_neighbours += 1
    # Return of 2 means seat becomes occupied
    if current_layout[x][y] == "L":
        if occupied_neighbours == 0:
            return 2
    # Return of 1 means seat becomes empty
    elif current_layout[x][y] == "#":
        if occupied_neighbours >= 4:
            return 1
    # Return of 0 means seat stays the way it is
    else:
        return 0


def seat_new_state_part_2(current_layout, x ,y):
    neighbours = get_surrounding_part_2(current_layout, x, y)
    if current_layout[x][y] == "L":
        if len(neighbours) == 0:
            return 2
    # Return of 1 means seat becomes empty
    elif current_layout[x][y] == "#":
        if len(neighbours) >= 5:
            return 1
    # Return of 0 means seat stays the way it is
    else:
        return 0


# Code
layout_1 = [[seat for seat in row] for row in inp]
layout_2 = [[seat for seat in row] for row in inp]

finished = [False, False]
while not (finished[0] and finished[1]):
    changed_1 = False
    changed_2 = False
    part_1 = 0
    part_2 = 0
    current_layout_1 = deepcopy(layout_1)
    current_layout_2 = deepcopy(layout_2)
    for x in range(0, len(layout_1)):
        for y in range(0, len(layout_1[0])):
            new_state_1 = seat_new_state_part_1(current_layout_1, x, y)
            if new_state_1 == 2:
                layout_1[x][y] = "#"
                changed_1 = True
            elif new_state_1 == 1:
                layout_1[x][y] = "L"
                changed_1 = True
            if layout_1[x][y] == "#":
                part_1 += 1
            new_state_2 = seat_new_state_part_2(current_layout_2, x, y)
            if new_state_2 == 2:
                layout_2[x][y] = "#"
                changed_2 = True
            elif new_state_2 == 1:
                layout_2[x][y] = "L"
                changed_2 = True
            if layout_2[x][y] == "#":
                part_2 += 1
    if not changed_1:
        finished[0] = True
    if not changed_2:
        finished[1] = True

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
