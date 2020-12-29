#!/usr/bin/env ruby
require 'set'

# Read in the file input
inp = File.open(ARGV[0]).read
# Variables for storing the results
part_1 = nil
part_2 = nil

def game(player_1, player_2, n, part)
  i = 0
  if part == 1
    until player_1.empty? or player_2.empty? do
      player_1_card = player_1.shift
      player_2_card = player_2.shift
      if player_1_card > player_2_card
        player_1 << player_1_card
        player_1 << player_2_card
      end
      if player_1_card < player_2_card
        player_2 << player_2_card
        player_2 << player_1_card
      end
      i += 1
    end
  elsif part == 2
    # Store previous decks
    backlog = Set.new
    until player_1.empty? or player_2.empty? do
      # Test if there has been a previous round with the exact same deck
      if backlog.include?(player_1.clone)
        return [1, player_1]
      elsif backlog.include?(player_2.clone)
        return [1, player_1]
      end
      backlog << player_1.clone
      backlog << player_2.clone
      player_1_card = player_1.shift
      player_2_card = player_2.shift
      if player_1.length >= player_1_card and player_2.length >= player_2_card
        # Make copies of the arrays with the number of cards = the value drawn
        player_1_tmp = player_1.clone[0..player_1_card-1]
        player_2_tmp = player_2.clone[0..player_2_card-1]
        sub_game = game(player_1_tmp, player_2_tmp, n+1, part)
        winning_player = sub_game[0]
        player_array = sub_game[1]
        if winning_player == 1
          player_1 << player_1_card
          player_1 << player_2_card
        end
        if winning_player == 2
          player_2 << player_2_card
          player_2 << player_1_card
        end
      else
        if player_1_card > player_2_card
          player_1 << player_1_card
          player_1 << player_2_card
        end
        if player_1_card < player_2_card
          player_2 << player_2_card
          player_2 << player_1_card
        end
      end
      i += 1
    end
  end
  player_array = 0
  winning_player = 0
  if player_1.length == 0
    winning_player = 2
    player_array = player_2
  elsif player_2.length == 0
    winning_player = 1
    player_array = player_1
  end

  return [winning_player, player_array]
end

def score(player_array)
  player_array = player_array.reverse()
  score = 0
  player_array.each.with_index do | card, index |
    score += card * (index+1)
  end
  return score
end

# Code
players = inp.split("\n\n")

player_1 = []
player_2 = []

player_array = [player_1, player_2]

players.each.with_index do | player_cards, player_n |
  player_cards = player_cards.split("\n")[1..]
  player_cards.each do | card |
    player_array[player_n] << card.to_i
  end
end

player_array_1 = game(player_1.clone, player_2.clone, 1, 1)[1]
part_1 = score(player_array_1)
player_array_2 = game(player_1.clone, player_2.clone, 1, 2)[1]
part_2 = score(player_array_2)

# Print the results
puts "Part 1: #{part_1}"
puts "Part 2: #{part_2}"
