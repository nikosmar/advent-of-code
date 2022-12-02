with open('nums.txt') as numfile:
    numlist = [int(line.strip()) for line in numfile
               if int(line.strip()) < 2020]

    numlist_hash = set(numlist)

    for number in numlist:
        if (2020 - number) in numlist_hash:
            print(number * (2020 - number))
            break
