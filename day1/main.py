elfs = []

file1 = open('input', 'r')
lines = file1.readlines()

cals = 0

for line in lines:
    if line.strip() == "":
        elfs.append(cals)
        cals = 0
        continue
    cals += int(line.strip())

elfs.sort(reverse=True)
print(elfs[0]+elfs[1]+elfs[2])