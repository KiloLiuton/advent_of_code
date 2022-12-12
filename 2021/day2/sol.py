import re

with open('input', 'r') as f:
    data = f.readlines()

def getRules(line):
    match = re.match("([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)", line)
    d = dict(raw = match.group(0),
             low  = int(match.group(1)),
             high = int(match.group(2)),
             char = match.group(3),
             pwd  = match.group(4))
    return d

i = 0  # part 1
j = 0  # part 2
for line in data:
    rules = getRules(line)
    password = rules['pwd']
    char = rules['char']
    if (password.count(char) > rules['high']) or (password.count(char) < rules['low']):
        i+=1
    if ((password[rules['low']-1] == char) + (password[rules['high']-1] == char)) == 1:
        j += 1

print(len(data) - i)
print(j)
