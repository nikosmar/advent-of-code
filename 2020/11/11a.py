def count_seats(seats_grid, column):
    adj_occupied_seats = 0

    for i in range(3):
        for j in range(-1, 2):
            if ((i-1) or j) and seats_grid[i][column + j] == '#':
                adj_occupied_seats += 1

    return adj_occupied_seats


with open("seatlayout.txt") as seatlayout:
    grid = []

    for line in seatlayout:
        grid.append('.' + line.strip() + '.')

    guard = '.' * len(grid[0])
    grid.insert(0, guard)
    grid.append(guard)

columns = len(grid[0]) - 1

buffer = []

change = True

while change:
    change = False

    for row in range(1, len(grid) - 1):
        buffer_line = '.'
        for col in range(1, columns):
            seat = grid[row][col]

            if seat != '.':
                adj_seats = count_seats(grid[row - 1: row + 2], col)

                if seat == 'L' and adj_seats == 0:
                    change = True
                    seat = '#'
                elif seat == '#' and adj_seats > 3:
                    change = True
                    seat = 'L'

            buffer_line += seat

        buffer_line += '.'

        if row > 2:
            grid[row - 2] = buffer[0]
            buffer[0] = buffer[1]
            buffer.pop()

        buffer.append(buffer_line)

    grid[row] = buffer.pop()
    grid[row - 1] = buffer.pop()

occupied_seats = 0

for row in grid:
    for seat in row:
        if seat == '#':
            occupied_seats += 1

print(occupied_seats)
