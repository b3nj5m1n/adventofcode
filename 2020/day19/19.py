#!/usr/bin/env python3
import fileinput
import re
import regex as reg
import uuid

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = None
part_2 = None

# Code
rules_tmp = [rule.replace("", "") for rule in inp[: inp.index("")]]
messages = inp[inp.index("") + 1 :]
rules = {}
for rule in rules_tmp:
    parts = rule.split(":")
    search = re.search('"(\w)"', parts[1])
    if not search:
        rules[int(parts[0])] = parts[1].split("|")
    else:
        rules[int(parts[0])] = search.group(1)


def regex(n, part):
    rule = rules[int(n)]
    if type(rule) == str:
        return re.match("(\w)", rule).group(1)
    if part == 2:
        if int(n) == 8:
            return f"({regex(42, part)}+)"
        if int(n) == 11:
            name = re.sub("\d|-", "", str(uuid.uuid4()))
            return f"(?P<{name}>{regex(42, part)}(?P>{name})?{regex(31, part)})"
    pattern = "|".join(
        ["".join([regex(sub, part) for sub in subrule.split()]) for subrule in rule]
    )
    return f"({pattern})"


def count(part):
    pattern = f"^{regex(0, part)}$"
    return len(reg.findall(pattern, "\n".join(messages), flags=reg.MULTILINE))


part_1 = count(1)
part_2 = count(2)

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
