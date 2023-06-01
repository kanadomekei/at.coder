n, x = map(int, input().split())

l = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
L = []
for i in l:
    for j in range(n):
        L.append(i)

print(L[x-1])


