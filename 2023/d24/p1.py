with open("input.txt", "r") as file:
    lines = file.read().splitlines()

count = 0
eqs = []

for line in lines:
    parts = line.split(" @ ")
    p = parts[0].split(", ")
    v = parts[1].split(", ")
    nx, ny = -int(v[1]), int(v[0])
    rx, ry = int(v[0]), int(v[1])
    p1, p2 = int(p[0]), int(p[1])
    a, b, c = nx, ny, nx * p1 + ny * p2
    eqs.append(((p1, p2), (rx, ry), (a, b, c)))

process = 0
size = len(eqs) - 1
l = (1 + size) * size / 2

for i in range(len(eqs)):
    for j in range(i + 1, len(eqs)):
        a, b, c = eqs[i][2]
        m, n, k = eqs[j][2]

        if a * n - b * m == 0 or m * b - a * n == 0:
            continue

        x = (n * c - b * k) / (a * n - b * m)
        y = (m * c - a * k) / (m * b - a * n)

        if (
            x < 200000000000000
            or x > 400000000000000
            or y < 200000000000000
            or y > 400000000000000
        ):
            continue

        ai, bi = eqs[i][0]
        aj, bj = eqs[j][0]
        rxi, ryi = eqs[i][1]
        rxj, ryj = eqs[j][1]
        if (x - ai) / rxi > 0 and (x - aj) / rxj > 0:
            count += 1

print("part one:", count)
