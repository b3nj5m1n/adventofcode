#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = "\n".join([line.strip() for line in fileinput.input()])
# Variables for storing the results
part_1 = 0
part_2 = 0


def in_range(number, lower, upper):
    if lower <= int(number) <= upper:
        return True
    return False


def valid(passport, part):
    if part == 1:
        for field in fields:
            if field[0] not in passport:
                if field[1]:
                    return False
        return True
    elif part == 2:
        if valid(passport, 1):
            if not in_range(passport["byr"], 1920, 2002):
                return False
            if not in_range(passport["iyr"], 2010, 2020):
                return False
            if not in_range(passport["eyr"], 2020, 2030):
                return False
            if passport["hgt"][-2:] == "cm":
                if not in_range(passport["hgt"][:-2], 150, 193):
                    return False
            elif passport["hgt"][-2:] == "in":
                if not in_range(passport["hgt"][:-2], 59, 76):
                    return False
            else:
                return False
            if not re.search("#([0-9a-fA-F]+){6}$", passport["hcl"]):
                return False
            if not re.search("^(amb|blu|brn|gry|grn|hzl|oth)$", passport["ecl"]):
                return False
            if not re.search("^(\d){9}$", passport["pid"]):
                return False
        else:
            return False
        return True


# Code
fields = [
    ["byr", True],
    ["iyr", True],
    ["eyr", True],
    ["hgt", True],
    ["hcl", True],
    ["ecl", True],
    ["pid", True],
    ["cid", False],
]

# Parse the input into indiviudal passports
passports = []
for passport in inp.split("\n\n"):
    attribs = {}
    for line in passport.splitlines():
        for keyvalue in line.split(" "):
            field, value = keyvalue.split(":")
            attribs[field] = value
    passports.append(attribs)
# Validate each passport
for passport in passports:
    if valid(passport, 1):
        part_1 += 1
    if valid(passport, 2):
        part_2 += 1

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
