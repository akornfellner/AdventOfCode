import sympy as sp

with open("input.txt", "r") as file:
    lines = file.read().splitlines()

cubes = []

for line in lines:
    parts = line.split(" @ ")
    p = parts[0].split(", ")
    v = parts[1].split(", ")
    cubes.append(
        (
            int(p[0]),
            int(p[1]),
            int(p[2]),
            int(v[0]),
            int(v[1]),
            int(v[2]),
        )
    )

x, y, z, vx, vy, vz = sp.symbols("x y z vx vy vz")

eqs = []
ts = []

for idx, cube in enumerate(cubes[:3]):
    x0, y0, z0, xv, yv, zv = cube
    t = sp.symbols("t" + str(idx))

    eqx = x + vx * t - x0 - xv * t
    eqy = y + vy * t - y0 - yv * t
    eqz = z + vz * t - z0 - zv * t

    eqs.append(eqx)
    eqs.append(eqy)
    eqs.append(eqz)
    ts.append(t)


sol = sp.solve(eqs, *([x, y, z, vx, vy, vz] + ts))
print(sol[0][0] + sol[0][1] + sol[0][2])
