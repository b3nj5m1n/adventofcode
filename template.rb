#!/usr/bin/env ruby
require 'set'

# Read in the file input
inp = File.open(ARGV[0]).readlines
# Variables for storing the results
part_1 = nil
part_2 = nil

# Code
inp.each { |line| puts line }

# Print the results
puts "Part 1: #{part_1}"
puts "Part 2: #{part_2}"
