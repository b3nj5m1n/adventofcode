#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = None
part_2 = None


def find_containers(bag, rules, part):
    result = []
    for key, value in rules.items():
        if part == 1:
            if bag in value:
                result.append(key)
        elif part == 2:
            if bag in [thing[0] for thing in value]:
                result.append([key, value[[thing[0] for thing in value].index(bag)][1]])
    return result


def compute(bag, rules, part):
    result = []
    bags = find_containers(bag, rules, part)
    for r in bags:
        result += compute(r, rules, part)
    result += bags
    return result


def solve_part_1(inp):
    rules = {}
    for rule in inp:
        if not rule == "":
            definitions = re.search("(\w+ \w+) bags contain (.+)", rule)
            rules[definitions.group(1)] = []
            for contains in definitions.group(2).split(", "):
                search = re.search("\d (\w+ \w+) bag", contains)
                if search:
                    rules[definitions.group(1)].append(search.group(1))
                else:
                    rules[definitions.group(1)] = []
    return len(set(compute("shiny gold", rules, 1)))


def num_of_descendants(bag, rules):
    if bag[0] == None:
        return 0
    result = 1
    print(bag)
    for sub_bag in rules[bag[0]]:
        result += 1 + num_of_descendants(sub_bag, rules)
    return result


def total_number(bag_set, rules):
    result = 0
    for bag in bag_set:
        result += 1 + num_of_descendants(bag, rules)
    return result


def get_descendants(bag, rules):
    if bag == "None":
        return []
    descendants = rules[bag]
    return descendants



def solve_part_2(inp):
    rules = {}
    for rule in inp:
        if not rule == "":
            definitions = re.search("(\w+ \w+) bags contain (.+)", rule)
            rules[definitions.group(1)] = []
            for contains in definitions.group(2).split(", "):
                search = re.search("(\d) (\w+ \w+) bag", contains)
                if search:
                    rules[definitions.group(1)].append([search.group(2), int(search.group(1))])
                else:
                    rules[definitions.group(1)].append([None, 0])
    bag = "shiny gold"
    count = 0
    stack = get_descendants(bag, rules)
    while len(stack):
        descendant = stack.pop()
        count += descendant[1]
        name = str(descendant[0])
        stack += (get_descendants(name, rules) * descendant[1])
    return count


# Code
part_1 = solve_part_1(inp)
part_2 = solve_part_2(inp)

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
