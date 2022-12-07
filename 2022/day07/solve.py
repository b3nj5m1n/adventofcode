#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = 0
part_2 = 0


dir_stack = []
listing = False
fs = {"/": {}}
for line in inp:
    if line.startswith("$"):
        listing = False
    match line.split(" "):
        case ["$", "cd", ".."]:
            dir_stack.pop()
        case ["$", "cd", d]:
            dir_stack.append(d)
        case ["$", "ls"]:
            listing = True
        case ["dir", d]:
            current_dir = fs.get(dir_stack[0])
            for sub_dir in dir_stack[1::]:
                current_dir = current_dir.get(sub_dir)
            current_dir[d] = {}
        case [size, filename]:
            current_dir = fs.get(dir_stack[0])
            for sub_dir in dir_stack[1::]:
                current_dir = current_dir.get(sub_dir)
            current_dir[filename] = size

dirs = []
size_total = 0
def get_dir_sizes(fname, fs):
    global size_total
    file_size = 0
    sub_dirs_size = 0
    for filename, content in fs.items():
        if type(content) is dict:
            x = get_dir_sizes(filename, content)
            sub_dirs_size += x
        else:
            file_size += int(content)
    size = sub_dirs_size + file_size
    size_total += file_size
    dirs.append((fname, size))
    return size
get_dir_sizes("/", fs["/"])

free_space = 70000000 - size_total
to_free = 30000000 - free_space
best_dir_to_free = ("not able to free", size_total)

for d in dirs:
    x = d[1]
    if x <= 100000:
        part_1 += d[1]
    if x > to_free and x < best_dir_to_free[1]:
        best_dir_to_free = d

part_2 = best_dir_to_free[1]

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
