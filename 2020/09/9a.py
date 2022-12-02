def sum_exists(sorted_list, desired_sum):
    start = 0
    end = len(sorted_list) - 1

    while start < end:
        if sorted_list[start] + sorted_list[end] < desired_sum:
            start += 1
        elif sorted_list[start] + sorted_list[end] > desired_sum:
            end -= 1
        else:
            return True

    return False


with open("numbers.txt") as numbers:
    data = [int(line.strip()) for line in numbers]

preamble_length = 25

for i in range(preamble_length, len(data)):
    preamble = sorted(data[i - preamble_length:i])

    if not sum_exists(preamble, data[i]):
        print(data[i])
    else:
        preamble.append(data[i])
