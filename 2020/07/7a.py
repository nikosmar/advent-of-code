import re


name_rgx = re.compile("[a-z]+ [a-z]+")
contains_rgx = re.compile("[0-9]+ [a-z]+ [a-z]+")


def find_bags_containing(color_name, used_bags):
    eligible_bags = 0

    colors_to_search = []

    with open('bagsrules.txt') as bags_file:
        for rule in bags_file:
            bag_color = name_rgx.findall(rule)[0]

            contained_colors = ' '.join(contains_rgx.findall(rule))

            if color_name in contained_colors and bag_color not in used_bags:
                eligible_bags += 1
                colors_to_search.append(bag_color)
                used_bags.append(bag_color)

    for color in colors_to_search:
        new_eligible_bags, used_bags = find_bags_containing(color, used_bags)
        eligible_bags += new_eligible_bags

    return eligible_bags, used_bags


print(find_bags_containing("shiny gold", [])[0])
