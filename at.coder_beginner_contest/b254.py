n = int(input())

l = [[1]]

for i in range(n-1):
    for j in range(n):
        s = range(j)
        l.append(s)

print(l)
        