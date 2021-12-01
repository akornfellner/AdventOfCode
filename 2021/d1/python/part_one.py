with open("input.txt", "r") as file:
    input = file.read()

numbers = [int(x) for x in input.split("\n")]

count = 0

for i in range(1, len(numbers)):
    if numbers[i] > numbers[i-1]:
        count += 1

print(count)
