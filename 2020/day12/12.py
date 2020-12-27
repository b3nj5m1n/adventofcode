#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line.strip() for line in fileinput.input()]
# Variables for storing the results
part_1 = None
part_2 = None


class xy:
    def __init__(self, x, y):
        self.x = int(x)
        self.y = int(y)

    def __str__(self):
        return f"North/South {self.x} | East/West {self.y}\nManhattan Distance: {abs(self.x) + abs(self.y)}"

    def add_x(self, xPlus):
        self.x += int(xPlus)

    def add_y(self, yPlus):
        self.y += int(yPlus)


def parse_input(inp):
    result = []
    for line in inp:
        search = re.search("([N|E|S|W|F|L|R])(\d+)", line)
        result.append([search.group(1), int(search.group(2))])
    return result


def turn(direction, degrees, current_direction):
    current_index = directions.index(current_direction)
    turns = int(degrees / 90)
    turn_direction = 1 if direction == "R" else -1
    # print(f"Current direction index: {current_index}, turns: {turns}, turn_direction: {turn_direction}")
    return directions[(current_index + (turns * turn_direction)) % len(directions)]


def move(direction, steps, current_position):
    if direction == "F":
        direction = current_direction
    x = current_position[0] + mappings_move[direction][0] * steps
    y = current_position[1] + mappings_move[direction][1] * steps
    return [x, y]


def execute_instruction(instruction_type, instruction_argument):
    global current_direction
    global current_position
    if instruction_type in ["R", "L"]:
        current_direction = turn(
            instruction_type, instruction_argument, current_direction
        )
    elif instruction_type in ["N", "E", "S", "W", "F"]:
        current_position = move(
            instruction_type, instruction_argument, current_position
        )


# Code
current_direction = "E"
mappings_move = {
    "N": [0, 1],
    "S": [0, -1],
    "E": [1, 0],
    "W": [-1, 0],
}
directions = ["N", "E", "S", "W"]
current_position = [0, 0]

for step in parse_input(inp):
    execute_instruction(step[0], int(step[1]))

part_1 = abs(current_position[0]) + abs(current_position[1])

ship = xy(0, 0)
waypoint = xy(1, 10)

for line in parse_input(inp):
    ins = line[0]
    arg = line[1]
    if ins == "N":
        waypoint.x += arg
    elif ins == "S":
        waypoint.x += -arg
    elif ins == "E":
        waypoint.y += arg
    elif ins == "W":
        waypoint.y += -arg
    elif ins == "F":
        for i in range(0, arg):
            ship.x += waypoint.x
            ship.y += waypoint.y
    elif ins == "L":
        for i in range(0, int(arg / 90)):
            temp = waypoint.x
            waypoint.x = waypoint.y
            waypoint.y = -temp
    elif ins == "R":
        for i in range(0, int(arg / 90)):
            temp = waypoint.x
            waypoint.x = -waypoint.y
            waypoint.y = temp

part_2 = abs(ship.x) + abs(ship.y)

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
