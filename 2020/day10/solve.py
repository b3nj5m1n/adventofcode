#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = None
part_2 = None


def possible_ways_to_end(index):
    # We're at the last adapter, there's only one way to get to the end
    if index == len(ratings)-1:
        return 1
    if index in possible_ways_to_end_dict:
        return possible_ways_to_end_dict[index]
    answer = 0
    # Loop over list and add possible paths
    for i in range(index+1, len(ratings)):
        # Add possible paths if the gap is less than or equal to 3
        if ratings[i] - ratings[index] <= 3:
            answer += possible_ways_to_end(i)
    possible_ways_to_end_dict[index] = answer
    return answer


# Code
ratings = sorted([int(adapter) for adapter in inp])
highest_rating = ratings[-1]
lowest_rating = ratings[0]

outlet_rating = 0
device_rating = highest_rating + 3

ratings.append(device_rating)

allowed_difference = 3

stack = [outlet_rating]

while not len(ratings) == 0:
    for i in range(len(ratings)):
        if ratings[i] in range(stack[-1]+1, stack[-1]+4):
            stack.append(ratings.pop(i))
            break

counts = {
    1: 0,
    2: 0,
    3: 0,
}
for i in range(len(stack)-1):
    counts[stack[i+1] - stack[i]] += 1

part_1 = (counts[1] * counts[3])

ratings = sorted([int(adapter) for adapter in inp])

ratings.append(0)
ratings.sort()
ratings.append(ratings[-1]+3)

possible_ways_to_end_dict = {}

part_2 = possible_ways_to_end(0)

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
