with open("input.txt", "r") as file:
    input = file.read()

numbers = [int(x) for x in input.split("\n")]

count = 0

for i in range(3, len(numbers)):
    old = [numbers[i-1], numbers[i-2], numbers[i-3]]
    new = [numbers[i], numbers[i-1], numbers[i-2]]

    if sum(new) > sum(old):
        count += 1

print(count)
