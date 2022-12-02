import re


def apply_mask(bit_mask, decimal_value, mem_address, memory):
    i = 0

    x_found = False

    for bit in bit_mask:
        if bit == '1':
            mem_address = mem_address[:i] + bit + mem_address[i + 1:]
        elif bit == 'X':
            bit_mask = bit_mask[:i] + '0' + bit_mask[i + 1:]

            mem_address = mem_address[:i] + '0' + mem_address[i + 1:]
            apply_mask(bit_mask, decimal_value, mem_address, memory)

            mem_address = mem_address[:i] + '1' + mem_address[i + 1:]
            apply_mask(bit_mask, decimal_value, mem_address, memory)

            x_found = True
            break
        i += 1

    if not x_found:
        memory[mem_address] = decimal_value

    return


mem = {}

with open("initprogram.txt") as init_program:
    for line in init_program:
        if line.split(' = ')[0] == "mask":
            mask = line.split(' = ')[1][:-1]
        else:
            address, value = re.findall("[0-9]+", line)
            # zero pad

            address = bin(int(address))[2:]
            address = (36 - len(address)) * "0" + address

            apply_mask(mask, int(value), address, mem)

print(sum(mem.values()))
