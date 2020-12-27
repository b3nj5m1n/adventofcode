#!/usr/bin/env python3
import fileinput
import re
import numpy as np

# Read in the file input
inp = "".join([line for line in fileinput.input()]).split("\n\n")
if "" in inp:
    inp.remove("")
# Variables for storing the results
part_1 = 1
part_2 = None

# Code
# An object representing one tile in the image
class Tile:
    def __init__(self, tile_id, content):
        # The tile_id holds the id of the tile
        self.tile_id = tile_id
        # The content holds an array of arrays of strings for each row in the tile
        self.content = content
        # If a tile is locked, it cannot be rotated / flipped any more
        self.locked = False
        # Neighbour variables to be populated by adjacent tiles
        self.north_neigh = None
        self.east_neigh = None
        self.south_neigh = None
        self.west_neigh = None

    def __str__(self):
        return self.tile_id

    def lock(self):
        self.locked = True

    def flip(self):
        if not self.locked:
            self.content = self.content[::-1]

    def rotate(self):
        if not self.locked:
            self.content = list(zip(*self.content[::-1]))
            self.content = ["".join(row) for row in self.content]


# List which holds all tiles
tiles = []
# Variable which will hold one of the corner tiles
starting_tile = None

# Parse the input into tiles
for soon_to_be_tile in inp:
    soon_to_be_tile = soon_to_be_tile.splitlines()
    # Exract the tile_id from the first line
    tile_id = re.search("Tile (\d+):", soon_to_be_tile[0]).group(1)
    # The rest of the array is the content
    content = soon_to_be_tile[1:]
    # Create a new Tile object and append it to the tiles list
    tiles.append(Tile(tile_id, content))

# Stack for tiles to be visited, this makes sure that we don't fuck up the orientation
stack = [tiles[0]]
# List for already paired up tiles
checked = set()
# Loop over tiles
while len(stack) > 0:
    tile = stack.pop()
    if tile.tile_id in checked:
        continue
    checked.add(tile.tile_id)
    # For every tile, loop over every tile
    for possible_neighbour in tiles:
        # If the possible neighbour is already checked, don't bother
        if possible_neighbour.tile_id in checked:
            continue
        # Loop over flips
        for _ in range(2):
            possible_neighbour.flip()
            # Loop over rotations
            for _ in range(4):
                possible_neighbour.rotate()
                # Check if the tile matches the side of the tile
                north_edge = tile.content[0]
                if north_edge == possible_neighbour.content[-1]:
                    possible_neighbour.lock()
                    tile.north_neigh = possible_neighbour
                    possible_neighbour.south_neigh = tile
                    stack.append(possible_neighbour)
                south_edge = tile.content[-1]
                if south_edge == possible_neighbour.content[0]:
                    possible_neighbour.lock()
                    tile.south_neigh = possible_neighbour
                    possible_neighbour.north_neigh = tile
                    stack.append(possible_neighbour)
                east_edge = "".join(list(zip(*tile.content))[-1])
                if east_edge == "".join([row[0] for row in possible_neighbour.content]):
                    possible_neighbour.lock()
                    tile.east_neigh = possible_neighbour
                    possible_neighbour.west_neigh = tile
                    stack.append(possible_neighbour)
                west_edge = "".join(list(zip(*tile.content))[0])
                if west_edge == "".join(
                    [row[-1] for row in possible_neighbour.content]
                ):
                    possible_neighbour.lock()
                    tile.west_neigh = possible_neighbour
                    possible_neighbour.east_neigh = tile
                    stack.append(possible_neighbour)

# Find a corner
for tile in tiles:
    neigh_count = 0
    if tile.north_neigh:
        neigh_count += 1
    if tile.east_neigh:
        neigh_count += 1
    if tile.south_neigh:
        neigh_count += 1
    if tile.west_neigh:
        neigh_count += 1
    if neigh_count == 2:
        part_1 *= int(str(tile))
    if (
        not tile.north_neigh == None
        and not tile.west_neigh == None
        and tile.east_neigh == None
        and tile.south_neigh == None
    ):
        starting_tile = tile

NESSIE_STR = """00000000000000000010
            10000110000110000111
            01001001001001001000"""


class Image:
    def __init__(self):
        self.tiles = {}
        self.x_max = 0
        self.y_max = 0

    def build_array(self):
        self.image = [
            [None for _ in range(self.x_max + 1)] for _ in range(self.y_max + 1)
        ]
        for x in range(self.x_max + 1):
            for y in range(self.y_max + 1):
                self.tiles[(x, y)].locked = False
                self.tiles[(x, y)].flip()
                self.tiles[(x, y)].rotate()
                self.tiles[(x, y)].flip()
                self.tiles[(x, y)].rotate()
                self.tiles[(x, y)].rotate()
                self.tiles[(x, y)].rotate()
                self.image[x][y] = self.tiles[(x, y)].content

    def build_string(self, remove_borders=True):
        result = ""
        # Loop over the rows of tiles
        for y in range(self.y_max + 1):
            # Loop over the lines in the tiles
            if not remove_borders:
                line_range = range(len(self.tiles[(0, 0)].content))
            else:
                line_range = range(1, len(self.tiles[(0, 0)].content) - 1)
            for k in line_range:
                # Loop over the tiles
                for x in range(self.x_max + 1):
                    if not remove_borders:
                        to_append = self.tiles[(x, y)].content[k]
                    else:
                        to_append = self.tiles[(x, y)].content[k][1:-1]
                    result += to_append
                result += "\n"
        self.ascii_image = result

    def rotate(self):
        arr = self.ascii_image
        arr = [list(row) for row in arr.splitlines()]
        arr = list(zip(*arr[::-1]))
        arr = "\n".join(["".join(row) for row in arr])
        self.ascii_image = arr

    def flip(self):
        arr = self.ascii_image
        arr = [list(row) for row in arr.splitlines()]
        arr = arr[::-1]
        arr = "\n".join(["".join(row) for row in arr])
        self.ascii_image = arr

    def __str__(self):
        return self.ascii_image

    def add_tile(self, coordinates, tile):
        self.tiles[coordinates] = tile
        if coordinates[0] > self.x_max:
            self.x_max = coordinates[0]
        if coordinates[1] > self.y_max:
            self.y_max = coordinates[1]

    def count_nessies(self):
        global NESSIE_STR
        # Sea monster as numpy array
        NESSIE = np.array([list(l.strip()) for l in NESSIE_STR.split("\n")]).astype(
            np.int
        )
        # Convert ascii string to numpy array
        bin_str = [
            list(l.strip())
            for l in self.ascii_image.replace("#", "1").replace(".", "0").split("\n")
        ]
        bin_str = bin_str[:-1]
        SEA = np.array(bin_str).astype(np.int)
        # Loop over flips
        for _ in range(2):
            # Loop over rotations
            for _ in range(4):
                nessie_count = 0
                for row in range(SEA.shape[0] - NESSIE.shape[0]):
                    for col in range(SEA.shape[1] - NESSIE.shape[1]):
                        if np.all(
                            SEA[
                                row : row + NESSIE.shape[0], col : col + NESSIE.shape[1]
                            ]
                            & NESSIE
                            == NESSIE
                        ):
                            nessie_count += 1
                if not nessie_count == 0:
                    return nessie_count
                else:
                    SEA = np.rot90(SEA)
            SEA = np.flip(SEA)

    def roughness(self):
        hashs = self.ascii_image.count("#")
        nessies = self.count_nessies()
        global NESSIE_STR
        hashs_per_nessie = NESSIE_STR.count("1")
        return hashs - (nessies * hashs_per_nessie)

    def solve(self):
        img.build_array()
        img.build_string()
        return self.roughness()


done = False
rows = [""]
current = starting_tile
y = 0
img = Image()
while not done:
    starter = current
    x = 0
    while current:
        coords = (x, y)
        img.add_tile(coords, current)
        current = current.west_neigh
        x += 1
    if starter.north_neigh:
        current = starter.north_neigh
        y += 1
    else:
        done = True

part_2 = img.solve()

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
