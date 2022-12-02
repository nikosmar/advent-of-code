from math import ceil


def divide(seat, min, max):
    middle = ceil((max + min) / 2)

    if seat[0] == 'L' or seat[0] == 'F':
        if len(seat) == 1:
            return min

        return divide(seat[1:], min, middle - 1)

    if seat[0] == 'B' or seat[0] == 'R':
        if len(seat) == 1:
            return max

        return divide(seat[1:], middle, max)


with open("boardingpasses.txt") as boarding_passes:
    max_seat_id = float('-inf')

    for line in boarding_passes:
        boarding_pass = line.strip()

        row = divide(boarding_pass[:7], 0, 127)
        col = divide(boarding_pass[7:], 0, 7)

        seat_id = 8 * row + col

        if seat_id > max_seat_id:
            max_seat_id = seat_id

    print(max_seat_id)
