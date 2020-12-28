#!/usr/bin/env ruby
require 'set'

# Read in the file input
inp = File.open(ARGV[0]).readlines[0]
# Variables for storing the results
part_1 = nil
part_2 = nil

class CircularLinkedList
  attr_reader :current, :head, :max, :min
  def initialize()
    @head = nil
    @current = @head
    @cups = Hash.new
    @max = 0
    @min = 0
  end
  def to_s
    # Store the current object to restore later
    original_current = @current
    # Move to the beginning of the list
    self.to_first
    # Result variable to return
    result = "#{@current}"
    self.next
    # Loop over the list until we reach the beginning again
    until @current == @head
      result += " - #{@current}"
      self.next
    end
    # Restore the original current
    @current = original_current
    return result
  end
  def to_a
    # Store the current object to restore later
    original_current = @current
    # Move to the beginning of the list
    self.to_first
    # Result variable to return
    result = [@current]
    self.next
    # Loop over the list until we reach the beginning again
    until @current == @head
      result.append(@current)
      self.next
    end
    # Restore the original current
    @current = original_current
    return result
  end
  # Sets the head to the current node
  def set_head
    @head = @current
  end
  # Sets current to the beginning of the list
  def to_first
    @current = @head
  end
  # Returns true and moves to given node if it exists, returns false otherwise
  def move(node_value, start_at_head=FALSE)
    if @cups.include?(node_value)
      @current = node_value
      return TRUE
    end
    return FALSE
  end
  # Finds the given node in the list, leaves the current node at that node if it finds it and returns TRUE, otherwise returns FALSE
  def find(node_value)
    # Store the current object to restore later
    original_current = @current
    result = self.move(node_value)
    # Restore the original current
    @current = original_current
    return result
  end
  # Move one node
  def next
    @current = @cups[@current]
  end
  # Inserts after the current node, leaves current at the new node
  def insert(new_node)
    # If this is the first node being inserted
    if @head == nil
      @head = new_node
      @cups[@head] = new_node
    # Else, insert the node normally
    else
      # Set the next node of the node to be inserted to the next node of the current node
      @cups[new_node] = @cups[@current]
      # Set the next node of the current node to the node to be inserted
      @cups[@current] = new_node
    end
    # Move current to the new node
    @current = new_node
    if new_node > @max
      @max = new_node
    end
    if new_node < @min
      @min = new_node
    end
  end
  # Inserts the element clockwise to the existing node
  def insert_at(new_node, neighbour_node)
    # Moves current to the node we want to insert after
    self.find(neighbour_node)
    # Insert the node
    self.insert(new_node)
  end
  # Removes clockwise of current, returns an array with the removed objects
  def remove(count_to_remove)
    result = []
    recompute_min_max = FALSE
    for n in 1..count_to_remove
      result.append(@cups[@current])
      if @cups[@current] == @max or @cups[@current] == @min
        recompute_min_max = TRUE
      end
      if @cups[@current] == @head
        @head = @cups[@cups[@current]]
      end
      val_to_delete = @cups[@current]
      # puts @cups[@current]
      # puts @cups
      @cups[@current] = @cups[@cups[@current]]
      @cups.delete(val_to_delete)
    end
    if recompute_min_max
      self.min_max
    end
    return result
  end
  # Returns the minimum and maximum value in the list in an array like this [min, max]
  def min_max
    # Store the current object to restore later
    original_current = @current
    start = @current
    result = [@current, @current]
    self.next
    until @current == start
      value = @current
      if value < result[0]
        result[0] = value
      end
      if value > result[1]
        result[1] = value
      end
      self.next
    end
    # Set the min max variables
    @min = result[0]
    @max = result[1]
    # Restore the original current
    @current = original_current
    return result
  end
end

def simulate_moves(cups, n)
  for i in 1..n
    current_label = cups.current
    picked_up = cups.remove(3)
    min = cups.min
    max = cups.max
    destination_label = current_label - 1
    until cups.move(destination_label)
      destination_label -= 1
      if destination_label < min
        destination_label = max
      end
    end
    # Because of the nature of the move subroutine, the current element in the list right now is the destination cup
    picked_up.each do | cup |
      cups.insert(cup)
    end
    # Use move to get back to the current cup
    cups.move(current_label)
    # Use next to go to the cup immediately clockwise of current
    cups.next
  end
end

def get_result(cups, part)
  res = nil
  if part == 1
    res = ""
    # Move to 1
    cups.move(1)
    # Now set the head of the list to the current node (1)
    cups.set_head
    # Now printing will start at 1, we get the result if we just cut of the first character
    cups.to_a[1..].each do | cup |
      res += cup.to_s
    end
  elsif part == 2
    cups.move(1)
    res = 1
    cups.next
    res *= cups.current
    cups.next
    res *= cups.current
  end
  return res
end

# Code
inp1 = CircularLinkedList.new
inp2 = CircularLinkedList.new
for i in 0..inp.length-2
  inp1.insert(inp[i].to_i)
  inp2.insert(inp[i].to_i)
end
inp1.to_first
min, max = inp2.min_max
for i in max+1..1000000
  inp2.insert(i)
end
inp2.to_first
inp2.move(1000000)
inp2.next


simulate_moves(inp1, 100)
part_1 = get_result(inp1, 1)
simulate_moves(inp2, 10000000)
part_2 = get_result(inp2, 2)

# Print the results
puts "Part 1: #{part_1}"
puts "Part 2: #{part_2}"
