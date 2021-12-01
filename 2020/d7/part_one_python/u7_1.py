# Functions


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

    tmp = []

    for i in l:
        for n in numbers:
            a = i.split(n)
            if len(a) > 1:
                tmp.append(a[1])

    l = tmp

    return {"name": name, "bags": l}


def search_bag(name, target):
    if name == target:
        return False

    if len(bags[name]) < 1:
        return False

    if target in bags[name]:
        return True

    found = False

    for i in bags[name]:
        found = found or search_bag(i, target)

    return found


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

count = 0

for i in bags:
    if search_bag(i, target):
        count += 1

print(count)
