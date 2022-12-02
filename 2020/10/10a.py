with open("adapters.txt") as adapters_input:
    adapters = [int(line.strip()) for line in adapters_input]

adapters.sort()

one_jolt_diff = 0
three_jolt_diff = 1

current_adapter_joltage = 0

for adapter in adapters:
    if adapter - current_adapter_joltage == 1:
        one_jolt_diff += 1
    elif adapter - current_adapter_joltage == 3:
        three_jolt_diff += 1

    current_adapter_joltage = adapter

print(one_jolt_diff * three_jolt_diff)
