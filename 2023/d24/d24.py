import sympy as sp

with open("input.txt", "r") as file:
    lines = file.read().splitlines()

x, y = sp.symbols("x y")
count = 0

eqs = []

for line in lines:
    parts = line.split(" @ ")
    p = parts[0].split(", ")
    v = parts[1].split(", ")
    nx, ny = -int(v[1]), int(v[0])
    rx, ry = int(v[0]), int(v[1])
    a, b = int(p[0]), int(p[1])
    eqs.append(((a, b), (rx, ry), nx * x + ny * y - a * nx - b * ny))

k = 0
n = len(eqs) - 1
l = (1 + n) * n / 2

for i in range(len(eqs)):
    for j in range(i + 1, len(eqs)):
        k += 1
        print(k, "/", l)
        sol = sp.solve([eqs[i][2], eqs[j][2]], [x, y])
        if (
            type(sol) != dict
            or sol[x] < 200000000000000
            or sol[x] > 400000000000000
            or sol[y] < 200000000000000
            or sol[y] > 400000000000000
        ):
            continue

        ai, bi = eqs[i][0]
        aj, bj = eqs[j][0]
        rxi, ryi = eqs[i][1]
        rxj, ryj = eqs[j][1]
        if (sol[x] - ai) / rxi > 0 and (sol[x] - aj) / rxj > 0:
            count += 1

print("part one:", count)
