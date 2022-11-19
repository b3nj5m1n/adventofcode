#!/usr/bin/env python3
import fileinput
import re

# Read in the file input
inp = [line for line in fileinput.input()]
# Variables for storing the results
part_1 = 0
part_2 = None


def solve_allergene(allergene, foods):
    # Find all foods that contain this allergene
    relevant_foods = []
    for food in foods:
        if allergene in food[1]:
            relevant_foods.append(set(food[0]))
    if len(relevant_foods) > 1:
        common = relevant_foods[0].intersection(*relevant_foods[1:])
    else:
        common = relevant_foods[0]
    if len(common) == 1:
        return list(common)[0]
    else:
        return False


# Code
foods = []
for food in inp:
    contains_alergenes = re.search("\(contains (.+)\)", food)
    alergenes = []
    if contains_alergenes:
        alergenes = contains_alergenes.group(1).split(", ")
        food = re.sub("\(contains .+\)", "", food)
    ingredients = food.split(" ")
    ingredients.remove("\n")
    foods.append([ingredients, alergenes])

# Dictionary holding each allergene with all possible ingredients
allergenes = {}
# List holding all ingredients
ingreds = []
# Loop over all foods
for food in foods:
    for allergene in food[1]:
        if not allergene in allergenes:
            allergenes[allergene] = [[ings, True] for ings in food[0]]
        else:
            allergenes[allergene].append([[ings, True] for ings in food[0]])
    # For each allergene, if it was present before, print the common elements of the list
    for allergene in food[1]:
        if allergene in allergenes:
            # Loop over ingredients in dict, if not in this foods ingredient list, remove from dict
            for i, ingredient in enumerate(allergenes[allergene]):
                if not ingredient[0] in food[0]:
                    allergenes[allergene][i][1] = False
    # For each ingredient, add it to the ingreds set
    [ingreds.append(ingred) for ingred in food[0]]

# Create a set of ingredients that contain allergenes
poisonous = set()
for allergene in allergenes.values():
    for ingred in allergene:
        if ingred[1] == True:
            poisonous.add(ingred[0])

# Loop over all ingredients and create a set of safe ingredients
safe = set()
for ingredient in ingreds:
    if not ingredient in poisonous:
        safe.add(ingredient)

# Loop over foods and count occurrences of safe ingredients
for i, food in enumerate(foods):
    for ingredient in food[0]:
        if ingredient in safe:
            part_1 += 1

# Remove the safe ingredients from foods
for i, food in enumerate(foods):
    for ingredient in enumerate(food[0]):
        if ingredient[1] in safe:
            foods[i][0].remove(ingredient[1])

# Create a set of allergens
allergenes = set()
[[allergenes.add(allergene) for allergene in food[1]] for food in foods]
allergenes = list(allergenes)

RESULT = {}
while len(allergenes) > 0:
    allergen = allergenes.pop(0)
    ingredient = solve_allergene(allergen, foods)
    if ingredient:
        RESULT[allergen] = ingredient
        # Loop over food and remove the ingredient
        for i, food in enumerate(foods):
            while ingredient in food[0]:
                foods[i][0].remove(ingredient)
    else:
        allergenes.append(allergen)

RESULT_LS = []
# Convert the set to a list of tuples in the form [allergen, ingredient]
for allergen, ingredient in RESULT.items():
    RESULT_LS.append((allergen, ingredient))
RESULT_LS.sort()
# Loop over tuples in alphabetical order, print out the ingredient
part_2 = ""
for allergen, ingredient in RESULT_LS:
    part_2 += f"{ingredient},"
part_2 = part_2[:-1]

# Print the results
print(f"Part 1: {part_1}")
print(f"Part 2: {part_2}")
