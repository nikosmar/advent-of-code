import re


def evaluate_password(fields_dictionary):
    valid_pass = True

    for i in fields_dictionary:
        valid_pass = valid_pass and fields_dictionary[i]
        fields_dictionary[i] = False

    if valid_pass:
        return 1

    return 0


with open('passwords.txt') as passfile:
    valid_passwords = 0

    required_fields = {'byr': False, 'iyr': False, 'eyr': False, 'hgt': False,
                       'hcl': False, 'ecl': False, 'pid': False}

    for line in passfile:
        if line != "\n":
            detected_fields = re.findall("byr|iyr|eyr|hgt|hcl|ecl|pid", line)

            for field in detected_fields:
                required_fields[field] = True
        else:
            valid_passwords += evaluate_password(required_fields)

    print(valid_passwords + evaluate_password(required_fields))
