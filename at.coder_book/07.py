n = int(input())
l = sorted(list(map(int, input().split())), reverse=True)
ans = 0
for i in range(n):
    if i % 2 == 0:
        ans += l[i]
    else:
        ans -= l[i]

print(ans)

