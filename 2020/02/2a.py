import re


with open('pass.txt') as passfile:
    valid_passwords = 0

    while True:
        line = passfile.readline()

        if not line:
            break

        data = re.findall('[0-9]+|[a-zA-Z]+', line)

        occurences = len(re.findall(data[2], data[3]))

        if int(data[0]) <= occurences <= int(data[1]):
            valid_passwords += 1

    print(valid_passwords)
