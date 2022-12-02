import re


def check_validity(field, rules):
    lower_bound = rules[0] <= field <= rules[1]
    upper_bound = rules[2] <= field <= rules[3]

    return lower_bound or upper_bound


my_ticket = None
nearby_tickets = []
mode = 0
error_rate = 0
rules = []

with open("tickets.txt") as puzzle_input:
    for line in puzzle_input:
        if line == "your ticket:\n":
            mode = 1
        elif line == "nearby tickets:\n":
            mode = 2
        elif line != "\n":
            if mode == 0:
                bounds = re.findall("[0-9]+", line)

                rules.append([int(bounds[0]), int(bounds[1]),
                              int(bounds[2]), int(bounds[3])])
            elif mode == 1:
                my_ticket = list(map(lambda x: int(x), line[:-1].split(',')))
            elif mode == 2:
                ticket = list(map(lambda x: int(x), line[:-1].split(',')))
                valid_ticket = True

                for field in ticket:
                    valid_field = False

                    for ticket_rule in rules:
                        if check_validity(field, ticket_rule):
                            valid_field = True
                            break

                    if not valid_field:
                        valid_ticket = False
                        error_rate += field
                        break

                if valid_ticket:
                    nearby_tickets.append(ticket)

print(f"Part 1: {error_rate}")

length = len(my_ticket)
pair_matrix = [[1 for _ in range(length)] for _ in range(length)]

for col in range(length):
    for ticket in nearby_tickets:
        for field_index in range(length):
            if not check_validity(ticket[col], rules[field_index]):
                pair_matrix[col][field_index] = 0

column_appearances = [0 for _ in range(length)]
for i in range(length):
    for j in range(length):
        column_appearances[j] += pair_matrix[i][j]

apps_rank = []
for i in range(length):
    apps_rank.append((column_appearances[i], i))

apps_rank.sort()
pairs = []
for tup in apps_rank:
    col = tup[1]

    for row in range(length):
        if pair_matrix[row][col] == 1:
            pairs.append((col, row))
            for j in range(length):
                pair_matrix[row][j] = 0
            break

pairs.sort()
departures = 1
for i in range(6):
    departures *= my_ticket[pairs[i][1]]
print(f"Part 2: {departures}")
