import math


ship_x_axis = 0
ship_y_axis = 0

waypoint_offset_x = 10
waypoint_offset_y = 1

waypoint_x_axis = ship_x_axis + waypoint_offset_x
waypoint_y_axis = ship_y_axis + waypoint_offset_y

# in radians
compass = {"N": 1.5707, "S": 4.7123, "E": 0, "W": 3.1415}

with open("instructions.txt") as instructions:
    for instruction in instructions:
        action = instruction[0]
        value = int(instruction[1:-1])

        if action == "L" or action == "R":
            rotation = value if action == "L" else value * -1
            rotation = math.radians(rotation)

            x = waypoint_offset_x
            y = waypoint_offset_y

            waypoint_offset_x = round(x * math.cos(rotation) - y * math.sin(rotation))
            waypoint_offset_y = round(x * math.sin(rotation) + y * math.cos(rotation))

            waypoint_x_axis = ship_x_axis + waypoint_offset_x
            waypoint_y_axis = ship_y_axis + waypoint_offset_y
        elif action == "F":
            ship_x_axis += value * waypoint_offset_x
            ship_y_axis += value * waypoint_offset_y

            waypoint_x_axis = ship_x_axis + waypoint_offset_x
            waypoint_y_axis = ship_y_axis + waypoint_offset_y
        else:
            angle = compass[action]

            waypoint_x_axis += round(math.cos(angle)) * value
            waypoint_y_axis += round(math.sin(angle)) * value

            waypoint_offset_x = waypoint_x_axis - ship_x_axis
            waypoint_offset_y = waypoint_y_axis - ship_y_axis

print(abs(ship_x_axis) + abs(ship_y_axis))
