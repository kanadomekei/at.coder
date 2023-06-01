def f(x):
    return  sum(list(map(int, str(x))))


n, s, t = map(int, input().split())
ans = 0
for i in range(1, n+1):
    if s <= f(i) <= t:
        ans += i

print(ans)

