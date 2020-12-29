#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = 0
part_2 = 1


class Rule:
    def __init__(self, name, range_1, range_2):
        self.name = name
        self.range_1 = range_1
        self.range_2 = range_2
        self.taken = False

    def __str__(self):
        return f"{self.name}: {self.range_1} or {self.range_2}"

    def is_valid(self, number):
        return (number in self.range_1 or number in self.range_2) and not self.taken

    def is_list_valid(self, list_of_numbers):
        valid = True
        for number in list_of_numbers:
            if not self.is_valid(number):
                valid = False
        return valid

    def set_taken(self):
        self.taken = True

    def get_taken(self):
        return self.taken

    def get_name(self):
        return self.name


class Field:
    def __init__(self, index):
        self.index = index
        self.numbers = []

    def __str__(self):
        return str(self.numbers)

    def add_number(self, number):
        self.numbers.append(number)

    def compute_valid(self, rules):
        self.valid = [False for _ in range(len(self.numbers))]
        self.rules = [None for _ in range(len(self.numbers))]
        for i, rule in enumerate(rules):
            valid = rule.is_list_valid(self.numbers)
            self.valid[i] = valid
            self.rules[i] = rule
        self.valid_count = sum(self.valid)


# Code
rules = []
my_ticket = None
tickets = []
iterator = 0
for line in inp:
    if line == "":
        iterator += 1
    if iterator == 0 and not line == "nearby tickets:" and not line == "":
        search = re.search("(.+): (\d+)-(\d+) or (\d+)-(\d+)", line)
        rules.append(
            Rule(
                search.group(1),
                range(int(search.group(2)), 1 + int(search.group(3))),
                range(int(search.group(4)), 1 + int(search.group(5))),
            )
        )
    if iterator == 1 and not line == "your ticket:" and not line == "":
        my_ticket = line.split(",")
    if iterator == 2 and not line == "nearby tickets:" and not line == "":
        tickets.append([int(field) for field in line.split(",")])

num_of_fields = len(rules)
fields = [Field(i) for i in range(0, num_of_fields)]

for ticket in tickets:
    for i, num in enumerate(ticket):
        valid = False
        for rule in rules:
            if rule.is_valid(num):
                valid = True
        if valid:
            fields[i].add_number(num)
        else:
            part_1 += ticket[i]

mapping = [None for _ in range(num_of_fields)]
while not sum(x is None for x in mapping) == 0:
    for field in fields:
        field.compute_valid(rules)
        if (field.valid_count) == 1:
            rule = field.rules[field.valid.index(True)]
            mapping[field.index] = rule
            rule.set_taken()
            break
for i, rule in enumerate(mapping):
    if "departure" in rule.get_name():
        part_2 *= int(my_ticket[i])

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
