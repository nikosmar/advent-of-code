puzzle_input = "9,19,1,6,0,5,4"

numbers = {}

turn = 1

for num in puzzle_input.split(','):
    numbers[int(num)] = turn
    turn += 1

last_spoken_num = int(num)

for turn in range(turn, 30000000 + 1):
    try:
        if numbers[last_spoken_num] < turn - 1:
            current_num = turn - 1 - numbers[last_spoken_num]
            numbers[last_spoken_num] = turn - 1
        else:
            current_num = 0
    except KeyError:
        numbers[last_spoken_num] = turn - 1
        current_num = 0

    last_spoken_num = current_num

print(last_spoken_num)
