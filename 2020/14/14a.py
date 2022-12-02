import re


def apply_mask(bit_mask, binary_value):
    # zero pad
    binary_value = (36 - len(binary_value)) * "0" + binary_value

    i = 0
    for bit in bit_mask:
        if bit != "X":
            binary_value = binary_value[:i] + bit + binary_value[i + 1:]
        i += 1

    return int(binary_value, 2)


mem = {}

with open("initprogram.txt") as init_program:
    for line in init_program:
        if line.split(' = ')[0] == "mask":
            mask = line.split(' = ')[1][:-1]
        else:
            address, value = re.findall("[0-9]+", line)
            mem[address] = apply_mask(mask, bin(int(value))[2:])

print(sum(mem.values()))
