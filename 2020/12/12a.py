import math


facing = 0

x_axis = 0
y_axis = 0

compass = {"N": 90, "S": 270, "E": 0, "W": 180}

with open("instructions.txt") as instructions:
    for instruction in instructions:
        action = instruction[0]
        value = int(instruction[1:-1])

        if action == "L" or action == "R":
            rotation = value if action == "L" else value * -1
            facing = (facing + rotation) % 360
        else:
            if action == "F":
                angle = math.radians(facing)
            else:
                angle = math.radians(compass[action])

            x_axis += round(math.cos(angle)) * value
            y_axis += round(math.sin(angle)) * value

print(abs(x_axis) + abs(y_axis))
