with open("bootcode.txt") as bc:
    bootcode = [[line.strip(), False] for line in bc]

pc = 0
cycle = []

# loop to detect the cycle
while True:
    op = bootcode[pc][0][:3]
    arg = int(bootcode[pc][0][4:])

    if bootcode[pc][1]:
        if cycle and [pc, op, arg] == cycle[0]:
            break
        else:
            cycle.append([pc, op, arg])

    bootcode[pc][1] = True

    if op == "jmp":
        pc += arg - 1

    pc += 1


# loop to detect the wrong command
for command in cycle:
    pc = command[0]
    initial_pc = pc

    op = command[1]
    initial_op = op

    arg = command[2]

    new_cycle = []

    if op != "acc":
        new_op = "jmp" if op == "nop" else "nop"
        bootcode[pc][0] = new_op + bootcode[pc][0][3:]

        while pc < len(bootcode) and (not new_cycle or pc != new_cycle[0]):
            new_cycle.append(pc)

            op = bootcode[pc][0][:3]
            arg = int(bootcode[pc][0][4:])

            if op == "jmp":
                pc += arg - 1

            pc += 1

    if pc < len(bootcode):
        bootcode[initial_pc][0] = initial_op + bootcode[initial_pc][0][3:]
    else:
        break

del cycle
del new_cycle

accumulator = 0
pc = 0

while pc < len(bootcode):
    op = bootcode[pc][0][:3]
    arg = int(bootcode[pc][0][4:])

    if op == "acc":
        accumulator += arg
    elif op == "jmp":
        pc += arg - 1

    pc += 1

print(accumulator)
