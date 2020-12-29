#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = 0
part_2 = 0


# Code
def new_eval(expression):
    search = re.search("(\d+\+\d+)", expression)
    while search:
        parts = search.group(1).split("+")
        s = str(sum([int(num) for num in parts]))
        expression = expression.replace(search.group(1), s, 1)
        search = re.search("(\d+\+\d+)", expression)
    search = re.findall("(\d+)", expression)
    result = 1
    for res in search:
        result *= int(res)
    return result


def old_eval(expression):
    matches = re.findall("(\+|\*|\d+)", expression)
    current_value = int(matches[0])
    operator = "+"
    for ins in matches[1:]:
        if ins in ["+", "*"]:
            operator = ins
        else:
            if operator == "+":
                current_value += int(ins)
            if operator == "*":
                current_value *= int(ins)
    return current_value


def eval_math(expression, part):
    while "(" in expression:
        last_parens = 0
        level = 0
        for i, char in enumerate(expression):
            if char == "(":
                if level == 0:
                    last_parens = i
                level += 1
            elif char == ")":
                level -= 1
                if level == 0:
                    sub_expression = expression[last_parens + 1 : i]
                    sub_solution = str(eval_math(sub_expression, part))
                    expression = expression.replace(f"({sub_expression})", sub_solution)
                    break
    return new_eval(expression) if part == 2 else old_eval(expression)


def eval_expression(expression, part):
    expression = expression.replace(" ", "")
    result = eval_math(expression, part)
    return result


for line in inp:
    part_1 += eval_expression(line, 1)
    part_2 += eval_expression(line, 2)

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
