#!/usr/bin/env ruby
require 'set'

# Read in the file input
inp = File.open(ARGV[0]).readlines
# Variables for storing the results
part_1 = 1
part_2 = "FREEBIE"

def perform_iteration(current_value, subject_number)
  current_value *= subject_number
  current_value = current_value % 20201227
  return current_value
end

def get_loop_size(public_key, subject_number)
  i = 0
  current = 1
  until current == public_key
    current = perform_iteration(current, subject_number)
    i += 1
  end
  return i
end

# Code
public_keys = [inp[0].to_i, inp[1].to_i]

subject_number = 7

loop_sizes = [
  get_loop_size(public_keys[0], subject_number),
  get_loop_size(public_keys[1], subject_number)
]

for i in 1..loop_sizes[0]
  part_1 = perform_iteration(part_1, public_keys[1])
end

# Print the results
puts "Part 1: #{part_1}"
puts "Part 2: #{part_2}"
