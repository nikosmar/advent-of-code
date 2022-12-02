with open('forest.txt') as forest:
    trees = [0, 0, 0, 0, 0]

    items_per_line = len(forest.readline()) - 1

    first_position_to_check = 1
    interval = 2

    even_line = False
    even_line_index = 1

    while True:
        line = forest.readline()

        if not line:
            break

        if even_line:
            if line[even_line_index] == '#':
                trees[4] += 1

            even_line_index = (even_line_index + 1) % items_per_line

        check = first_position_to_check
        for i in range(4):
            check = check % items_per_line

            if line[check] == '#':
                trees[i] += 1

            check += interval

        interval += 2
        first_position_to_check = (first_position_to_check + 1) % items_per_line
        even_line = not even_line

    product = 1
    for num in trees:
        product *= num
    print(product)
