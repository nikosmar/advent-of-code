with open('forest.txt') as forest:
    trees = 0

    items_per_line = len(forest.readline()) - 1

    position_to_check = 3

    while True:
        line = forest.readline()

        if not line:
            break

        position_to_check = position_to_check % items_per_line

        if line[position_to_check] == '#':
            trees += 1

        position_to_check += 3

    print(trees)
