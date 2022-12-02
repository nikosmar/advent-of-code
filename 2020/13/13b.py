from math import floor


def euclid2(a, b):
    if b == 0:
        return a, 1, 0

    d, x, y = euclid2(b, a % b)
    return d, y, x - y * floor(a / b)


with open("notes.txt") as notes:
    notes.readline()
    bus_lines = notes.readline().strip().split(',')

crt = []
offset = 0

for bus_line in bus_lines:
    if bus_line != 'x':
        bus_line = int(bus_line)
        crt.append([bus_line, (bus_line - offset) % bus_line])

    offset += 1

n = 1
for n_i in crt:
    n *= n_i[0]

for n_i in crt:
    n_i.append(n // n_i[0])

Y = 0
for item in crt:
    m_i = item[2]
    c_i = m_i * (euclid2(m_i, item[0])[1] % item[0])
    Y += c_i * item[1]

print(Y % n)
