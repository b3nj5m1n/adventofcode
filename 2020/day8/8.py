#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
ORIGINAL_INPUT = [line.strip() for line in fileinput.input()]
ORIGINAL_INPUT.append("")
# Variables for storing the results
part_1 = 0
part_2 = 0


def acc(value, current_line, current_acc):
    current_acc += int(value)
    return [current_line + 1, current_acc]


def jmp(value, current_line, current_acc):
    current_line += int(value)
    return [current_line, current_acc]


def nop(value, current_line, current_acc):
    return [current_line + 1, current_acc]


def execute(INPUT, current_line, current_acc, lines, switch=False):
    if INPUT[current_line] == "":
        return False
    if current_line in lines:
        return False
    lines.append(current_line)
    statement = INPUT[current_line]
    search = re.search("(\w+) ((\+|-)\d+)", statement)
    instruction = search.group(1)
    return function_mappings[instruction](
        search.group(2), current_line, current_acc
    ) + [lines]


# Code
function_mappings = {
    "acc": acc,
    "jmp": jmp,
    "nop": nop,
}

lines = []
line = 0
accumulator = 0
while True:
    res = execute(ORIGINAL_INPUT, line, accumulator, lines)
    if res:
        line, accumulator, lines = res
    else:
        break

part_1 = accumulator

INPUTS = []
for i in range(len(ORIGINAL_INPUT)):
    tmp = ORIGINAL_INPUT.copy()
    if tmp[i].split(" ")[0] == "jmp":
        tmp[i] = tmp[i].replace("jmp", "nop")
    INPUTS.append(tmp)

for INPUT in INPUTS:
    lines = []
    line = 0
    accumulator = 0
    call_stack = []
    while True:
        res = execute(INPUT, line, accumulator, lines)
        if res:
            line, accumulator, lines = res
        else:
            if line == len(ORIGINAL_INPUT) - 1:
                part_2 = accumulator
            break

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
