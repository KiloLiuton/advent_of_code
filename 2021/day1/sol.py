with open('input', 'r') as f:
    data = [int(x) for x in f.readlines()]

data = sorted(data)

for i in range(len(data)):
    for j in range(i+1, len(data)):
        if (data[i]+data[j]) == 2020:
            print(data[i], data[j], data[i]+data[j], data[i]*data[j])
            break
