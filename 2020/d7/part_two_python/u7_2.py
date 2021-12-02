# Functions


from types import resolve_bases


def split_bags(input):
    l = input.split(" bags")
    name = l[0]

    tmp = []

    numbers = [" 1 ", " 2 ", " 3 ", " 4 ", " 5 ", " 6 ", " 7 ", " 8 ", " 9 "]

    for i in l[1:]:
        a = i.split(" bag")
        for j in a:
            tmp.append(j)

    l = tmp

    tmp = {}

    for i in l:
        for n in numbers:
            a = i.split(n)
            if len(a) > 1:
                tmp[a[1]] = int(n[1])

    l = tmp

    return {"name": name, "bags": l}


def count_bags(name):
    if len(bags[name]) == 0:
        return 1

    result = 1

    for bag, number in bags[name].items():
        result += count_bags(bag) * number

    return result


# Main Code
with open("input.txt", "r") as file:
    input = file.read()


lines = input.split("\n")

tmp = []
for i in lines:
    tmp.append(split_bags(i))

bags = {}

for i in tmp:
    bags[i["name"]] = i["bags"]

target = "shiny gold"

print(count_bags(target) - 1)
