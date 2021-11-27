def size_from_line(line):
    values = line.split("x")

    l = int(values[0])
    w = int(values[1])
    h = int(values[2])

    m = max(l, w, h)

    return 2*(l+w+h)-2*m+l*w*h


with open("input.txt", "r") as file:
    input = file.read()

lines = input.split("\n")

result = 0

for line in lines:
    result += size_from_line(line)

print(result)
