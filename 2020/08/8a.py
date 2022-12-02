with open("bootcode.txt") as bc:
    bootcode = [[line.strip(), False] for line in bc]

accumulator = 0
pc = 0

while not bootcode[pc][1]:
    bootcode[pc][1] = True

    op = bootcode[pc][0][:3]
    arg = int(bootcode[pc][0][4:])

    if op == "acc":
        accumulator += arg
    elif op == "jmp":
        pc += arg - 1

    pc += 1

print(accumulator)
