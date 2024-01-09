with open("input.txt", "r") as file:
    data = file.read().splitlines()

lines = {}

for line in data:
    line = line.split(": ")
    lines[line[0]] = line[1]


def value(key):
    try:
        v = int(lines[key])
        return v
    except:
        left = lines[key][:4]
        right = lines[key][7:]
        if lines[key][5] == "+":
            return value(left) + value(right)
        elif lines[key][5] == "*":
            return value(left) * value(right)
        elif lines[key][5] == "-":
            return value(left) - value(right)
        elif lines[key][5] == "/":
            return value(left) / value(right)


print("part one:", int(value("root")))

# Part Two

from sympy import *

x = symbols("x")


def value(key, x):
    if key == "humn":
        return x
    try:
        v = int(lines[key])
        return v
    except:
        left = lines[key][:4]
        right = lines[key][7:]
        result = 0
        if lines[key][5] == "+":
            return value(left, x) + value(right, x)
        elif lines[key][5] == "*":
            return value(left, x) * value(right, x)
        elif lines[key][5] == "-":
            return value(left, x) - value(right, x)
        elif lines[key][5] == "/":
            return value(left, x) / value(right, x)


left = lines["root"][:4]
right = lines["root"][7:]

eq = Eq(value(left, x), value(right, x))
print("part one:", int(solve(eq, x)[0].round(0)))
