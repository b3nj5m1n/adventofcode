#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = 0
part_2 = 0


def apply_rule_part_1(test_case):
    rule, content = test_case.split(":")
    rule = rule.split(" ")
    rule = [rule[0].split("-"), rule[1]]
    count = 0
    for char in content:
        if char == rule[1]:
            count += 1
    if count >= int(rule[0][0]) and count <= int(rule[0][1]):
        return True
    return False


def apply_rule_part_2(test_case):
    rule, content = test_case.split(":")
    content = content.strip()
    rule = rule.split(" ")
    rule = [rule[0].split("-"), rule[1]]
    count = 0
    for index in rule[0]:
        if content[int(index)-1] == rule[1]:
            count += 1
    if count == 1:
        return True
    return False


# Code
for test_case in inp:
    if apply_rule_part_1(test_case):
        part_1 += 1
    if apply_rule_part_2(test_case):
        part_2 += 1

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
