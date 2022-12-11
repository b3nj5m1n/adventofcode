#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = "".join([line for line in fileinput.input()])
# Variables for storing the results
part_1 = 0
part_2 = 0

# Code
groups = []
current_group = []
for group in inp.split("\n\n"):
    groups.append([inp for inp in group.splitlines()])

groups_answers = []
for group in groups:
    answers = []
    for answer in group:
        for char in answer:
            answers.append(char)
    answer_dict = {}
    answer_dict["people"] = len(group)
    for answer in answers:
        if answer in answer_dict:
            answer_dict[answer] += 1
        else:
            answer_dict[answer] = 1
    groups_answers.append(answer_dict)


for answer_dict in groups_answers:
    part_1 += len(answer_dict)-1
    for key, value in answer_dict.items():
        if not key == "people" and value == answer_dict["people"]:
            part_2 += 1

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
