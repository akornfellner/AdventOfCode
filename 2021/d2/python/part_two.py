def split_line(line):
    values = line.split(" ")
    return {"cmd": values[0], "number": int(values[1])}


with open("input.txt", "r") as file:
    input = file.read()

h = 0
d = 0
aim = 0

commands = [split_line(x) for x in input.split("\n")]

for cmd in commands:
    if cmd["cmd"] == "forward":
        h += cmd["number"]
        d += cmd["number"] * aim
    elif cmd["cmd"] == "down":
        aim += cmd["number"]
    elif cmd["cmd"] == "up":
        aim -= cmd["number"]

print(h * d)
