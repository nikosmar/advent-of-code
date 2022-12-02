target = 1309761972

with open("numbers.txt") as numbers:
    data = [int(line.strip()) for line in numbers]

end = False

for i in range(len(data)):
    contiguous_set = [data[i]]
    sum_of_set = data[i]

    for j in range(i + 1, len(data)):
        sum_of_set += data[j]
        contiguous_set.append(data[j])

        if sum_of_set > target:
            break
        elif sum_of_set == target:
            end = True
            break

    if end:
        print(min(contiguous_set) + max(contiguous_set))
        break
