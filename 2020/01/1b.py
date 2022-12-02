with open('nums.txt') as numfile:
    numlist = [int(line.strip()) for line in numfile
               if int(line.strip()) < 2020]

    numlist_hash = set(numlist)

    next_index = 1
    for number0 in numlist:
        for number1 in numlist[next_index:]:
            s = number0 + number1
            if (2020 - s) in numlist_hash:
                print(number0 * number1 * (2020 - s))
                break

        next_index += 1
