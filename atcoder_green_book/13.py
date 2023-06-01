n, x = map(int, input().split())

l = []
for _ in range(n):
    s, t = map(int, input().split())
    t = int(t)
    l.append([s, t])

ans = -1
sum = 0

for i in range(0, n):
    sum += l[i][0] * l[i][1] 
    if 100 * x < sum:
        ans = i + 1
        break

print(ans)