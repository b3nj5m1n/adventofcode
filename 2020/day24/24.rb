#!/usr/bin/env ruby
require 'set'

# Read in the file input
inp = File.open(ARGV[0]).readlines
# Variables for storing the results
part_1 = nil
part_2 = nil

def count_black_tiles(tiles)
  tiles.length
end

def surrounding(coordinates)
  x, y, z = coordinates
  result = []
  $moves.each do |direction|
    i, j, k = direction[1]
    result.append([x + i, y + j, z + k])
  end
  result
end

# Code
$moves = {
  'e' => [1, 0, -1],
  'se' => [0, 1, -1],
  'sw' => [-1, 1, 0],
  'w' => [-1, 0, 1],
  'nw' => [0, -1, 1],
  'ne' => [1, -1, 0]
}
# Black = TRUE
# WHITE = FALSE
tiles = Set.new

inp.each do |line|
  line = line.split('')
  ins = []
  cords = [0, 0, 0]
  until line.length == 0
    char = line.shift
    ins.append(char)
    next unless %w[w e].include?(char)

    move = $moves[ins.join('')]
    cords = [cords[0] + move[0], cords[1] + move[1], cords[2] + move[2]]
    ins = []
  end
  if !tiles.include?(cords)
    tiles << cords
  else
    tiles.delete(cords)
  end
end

part_1 = count_black_tiles(tiles)

size = 4

for day in 1..100
  tmpg = tiles.clone
  # A set of tiles that need to be checked
  check = Set.new
  tiles.each do |tile|
    check << tile
    surrounding(tile).each do |neighbour|
      check << neighbour
    end
  end
  check.each do |tile|
    black_count = 0
    surrounding(tile).each do |neighbour|
      black_count += 1 if tmpg.include?(neighbour)
    end
    tiles.delete(tile) if tmpg.include?(tile) && ![1, 2].include?(black_count)
    tiles << tile if (tmpg.include?(tile) != 0) && (black_count == 2)
  end
end

part_2 = count_black_tiles(tiles)

# Print the results
puts "Part 1: #{part_1}"
puts "Part 2: #{part_2}"
