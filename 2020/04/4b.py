import re


def evaluate_password(fields_dictionary):
    valid_pass = True

    for i in fields_dictionary:
        valid_pass = valid_pass and fields_dictionary[i]
        fields_dictionary[i] = False

    if valid_pass:
        return 1

    return 0


rgx = "((byr:)(19[2-9][0-9]|200[0-2]))|"
rgx += "((iyr:)(201[0-9]|2020))|"
rgx += "((eyr:)(202[0-9]|2030))|"
rgx += "((hgt:)(((1([5-8][0-9]|9[0-3]))cm)|((59|6[0-9]|7[0-6])in)))|"
rgx += "((hcl:)(#[0-9a-f]{6}[^0-9a-f]))|"
rgx += "((ecl:)(amb|blu|brn|gry|grn|hzl|oth))|"
rgx += "((pid:)([0-9]{9}[^0-9]))"


compiled_rgx = re.compile(rgx)


with open('passwords.txt') as passfile:
    valid_passwords = 0

    required_fields = {'byr': False, 'iyr': False, 'eyr': False, 'hgt': False,
                       'hcl': False, 'ecl': False, 'pid': False}

    for line in passfile:
        if line != "\n":
            detected_fields = compiled_rgx.findall(line)

            for field_tuple in detected_fields:
                field = list(filter(None, field_tuple))[0][:3]
                required_fields[field] = True
        else:
            valid_passwords += evaluate_password(required_fields)

    print(valid_passwords + evaluate_password(required_fields))
