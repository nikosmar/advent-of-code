with open("notes.txt") as notes:
    earliest_departure = int(notes.readline().strip())
    bus_lines = notes.readline().strip().split(',')

bus_lines = list(filter(lambda _: False if _ == 'x' else True, bus_lines))

min_wait_time = float('inf')
earliest_bus_id = 0

for bus_line in bus_lines:
    bus_line = int(bus_line)

    # it's assumed that no bus departs at earliest_departure timestamp
    # i.e. earliest_departure % bus_line is never 0
    bus_departure = ((earliest_departure // bus_line) + 1) * bus_line
    wait_time = bus_departure - earliest_departure

    if wait_time < min_wait_time:
        min_wait_time = wait_time
        earliest_bus_id = bus_line


print(min_wait_time * earliest_bus_id)
