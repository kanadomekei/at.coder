import collections
n = int(input())
l = [''.join(sorted(input())) for _ in range(n)]
c = collections.Counter(l)
ans = 0

for i in c.values():
    ans += i * (i - 1) // 2

print(ans)



5