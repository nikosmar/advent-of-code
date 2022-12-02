import re


name_rgx = re.compile("[a-z]+ [a-z]+")
contains_rgx = re.compile("[0-9]+ [a-z]+ [a-z]+")


def find_bags_containing(color_name):
    total_bags = 0

    with open('bagsrules.txt') as bags_file:
        for rule in bags_file:
            bag_color = name_rgx.findall(rule)[0]

            contained_bags = contains_rgx.findall(rule)

            if bag_color == color_name:
                if contained_bags:
                    for bags in contained_bags:
                        contained_bag_quantity, contained_bag_color = re.findall("[0-9]+|[a-z]+ [a-z]+", bags)

                        total_bags += int(contained_bag_quantity)

                        total_bags += int(contained_bag_quantity) * find_bags_containing(contained_bag_color)
                    break
                else:
                    return 0

    return total_bags


print(find_bags_containing("shiny gold"))
