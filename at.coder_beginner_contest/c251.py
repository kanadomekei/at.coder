n = int(input())

string = 0
point = 1

l = []
for _ in range(n):
    s, t  = map(int, input().split())
    l.append([s, t])

ans = 0

for i in range(0, n-1):
    for j in range(i+1, n):
        if(l[i][string] == l[j][string]):
            l[j][string] = -1

max = -1
ans = 0

for i in range(n, 0):
    if max < l[i][point]:
        ans = i

print(i)





