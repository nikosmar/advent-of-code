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
    detected_seat_id_sum = 0

    max_seat_id = float('-inf')
    min_seat_id = float('inf')

    for line in boarding_passes:
        boarding_pass = line.strip()

        row = divide(boarding_pass[:7], 0, 127)
        col = divide(boarding_pass[7:], 0, 7)

        seat_id = 8 * row + col

        if seat_id < min_seat_id:
            min_seat_id = seat_id

        if seat_id > max_seat_id:
            max_seat_id = seat_id

        detected_seat_id_sum += seat_id

    # sum of a sequence of natural numbers
    expected_seat_id_sum = (max_seat_id * (max_seat_id + 1) -
                            min_seat_id * (min_seat_id - 1)) / 2

    print(int(expected_seat_id_sum) - detected_seat_id_sum)
