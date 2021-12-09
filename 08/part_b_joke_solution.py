num_to_num7 = [0b01110111, 0b00010010, 0b01011101, 0b01011011, 0b00111010,
               0b01101011, 0b01101111, 0b01010010, 0b01111111, 0b01111011]
num7_to_num = {0b01110111: 0, 0b00010010: 1, 0b01011101: 2, 0b01011011: 3, 0b00111010: 4,
               0b01101011: 5, 0b01101111: 6, 0b01010010: 7, 0b01111111: 8, 0b01111011: 9}
result = 0

with open("in.txt") as notes:
    for line in notes:
        letter_to_num = {'a': 0, 'b': 0, 'c': 0, 'd': 0, 'e': 0, 'f': 0, 'g': 0}
        inp, output = line.split('|')
        words_per_length = [[] for _ in range(6)]

        for number in inp.split():
            words_per_length[len(number) - 2].append(number)

        # find letters used to display 1 (positions ur, dr)
        for letter in words_per_length[2 - 2][0]:
            letter_to_num[letter] = num_to_num7[1]

        # find letter in position 'up' used to display 7
        for letter in words_per_length[3 - 2][0]:
            if letter_to_num[letter] == 0:
                letter_to_num[letter] = 64

        # find letters (positions ul, md) used to display 4
        for letter in words_per_length[4 - 2][0]:
            if letter_to_num[letter] == 0:
                letter_to_num[letter] = 32 + 8

        unknown_letters = 0
        for letter in words_per_length[5 - 2][0]:
            if letter_to_num[letter] == 0:
                unknown_letters += 1

        # number 2
        if unknown_letters == 2:
            for letter in words_per_length[5 - 2][0]:
                # since 2 and 1 have only 'ur' in common, we can deduct 'ur' (now) and 'dr' (later)
                if letter_to_num[letter] == num_to_num7[1]:
                    letter_to_num[letter] = 0b00010000
            for letter in words_per_length[5 - 2][0]:
                if letter_to_num[letter] == 0:
                    # marking 'dl' and 'dw'
                    letter_to_num[letter] = 5
                elif not (letter_to_num[letter] == 16 or letter_to_num[letter] == 64):
                    # deducting 'md'
                    letter_to_num[letter] = 0b00001000

            for letter, value in letter_to_num.items():
                if value == num_to_num7[1]:
                    letter_to_num[letter] = 0b00000010
                elif value == 40:
                    letter_to_num[letter] = 32

            for letter in words_per_length[5 - 2][1]:
                if letter_to_num[letter] == 5:
                    letter_to_num[letter] = 1

            for letter, value in letter_to_num.items():
                if value == 5:
                    letter_to_num[letter] = 4
        # number 3 or 5
        else:
            unknown_letters = 0

            for letter in words_per_length[5 - 2][0]:
                if letter_to_num[letter] == num_to_num7[1]:
                    unknown_letters += 1

            # number 3
            if unknown_letters == 2:
                for letter in words_per_length[5 - 2][0]:
                    if letter_to_num[letter] == 40:
                        letter_to_num[letter] = 8
                    elif letter_to_num[letter] == 0:
                        letter_to_num[letter] = 1

                for letter, value in letter_to_num.items():
                    if value == 40:
                        letter_to_num[letter] = 32
                    elif value == 0:
                        letter_to_num[letter] = 4

                next_is_5 = False
                add = 16
                for letter in words_per_length[5 - 2][1]:
                    if letter_to_num[letter] == 32:
                        next_is_5 = True

                if next_is_5:
                    add = 2

                for letter in words_per_length[5 - 2][1]:
                    if letter_to_num[letter] == 18:
                        letter_to_num[letter] = add

                for letter, value in letter_to_num.items():
                    if value == 18:
                        letter_to_num[letter] = 18 - add
            else:
                # number 5
                for letter in words_per_length[5 - 2][0]:
                    if letter_to_num[letter] == 18:
                        letter_to_num[letter] = 2
                    elif letter_to_num[letter] == 0:
                        letter_to_num[letter] = 1

                for letter, value in letter_to_num.items():
                    if value == 18:
                        letter_to_num[letter] = 16
                    elif value == 0:
                        letter_to_num[letter] = 4

                for letter in words_per_length[5 - 2][1]:
                    if letter_to_num[letter] == 40:
                        letter_to_num[letter] = 8

                for letter, value in letter_to_num.items():
                    if value == 40:
                        letter_to_num[letter] = 32

        output = output.split()
        weight = 1000
        for number in output:
            num = 0
            for char in number:
                num += letter_to_num[char]
            result += num7_to_num[num] * weight
            weight /= 10

print(result)
