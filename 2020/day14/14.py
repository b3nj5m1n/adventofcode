#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = 0
part_2 = 0


def get_possible_addresses(address):
    result = ["", ""]
    if "X" in address:
        found_x = False
        for i in range(len(address)):
            if not address[i] == "X":
                result[0] += (address[i])
                result[1] += (address[i])
            elif not found_x:
                result[0] += ("0")
                result[1] += ("1")
                found_x = True
            else:
                result[0] += (address[i])
                result[1] += (address[i])
    RESULT = []
    for i in range(len(result)):
        if "X" in result[i]:
            RESULT += (get_possible_addresses(result[i]))
        else:
            RESULT += ([result[i]])
    return RESULT


def do_block(inp, memory, part):
    if part == 2:
        mask = inp[0].split("=")[1].strip()
        for instr in inp[1:]:
            search = re.search("mem\[(\d+)]", instr)
            address = int(search.group(1))
            binary_address = "{0:b}".format(int(address)).zfill(len(mask))
            address_binary_string = ""
            for i in range(len(mask)):
                if mask[i] == 'X':
                    address_binary_string += (mask[i])
                elif mask[i] == '1':
                    address_binary_string += "1"
                else:
                    address_binary_string += (binary_address[i])
            addresses = get_possible_addresses(address_binary_string)
            value = int(instr.split("=")[1].strip())
            for address in addresses:
                memory[int(address, 2)] = value
        return memory
    if part == 1:
        mask = inp[0].split("=")[1].strip()
        for instr in inp[1:]:
            binary_string = "{0:b}".format(int(instr.split("=")[1].strip())).zfill(len(mask))
            result_binary_string = ""
            for i in range(len(mask)):
                if mask[i] == 'X':
                    result_binary_string += (binary_string[i])
                else:
                    result_binary_string += (mask[i])
            search = re.search("mem\[(\d+)]", instr)
            address = int(search.group(1))
            memory[address] = int(result_binary_string, 2)
        return memory


# Code
memory = {}
blocks = []
current_block = []
for line in inp:
    if re.search("mask", line):
        if not current_block == []:
            blocks.append(current_block)
        current_block = []
    current_block.append(line)
blocks.append(current_block)
memory_1 = memory.copy()
blocks_1 = blocks.copy()
for block in blocks_1:
    memory_1 = do_block(block, memory_1, 1)
for key, value in memory_1.items():
    part_1 += value
memory_2 = memory.copy()
blocks_2 = blocks.copy()
for block in blocks_2:
    memory_2 = do_block(block, memory_2, 2)
for key, value in memory_2.items():
    part_2 += value

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
