def f(x):
    if '7' not in str(x) and '7' not in str(oct(x)):
        return 1
    return 0

n = int(input())
ans = 0
for i in range(1, n+1):
    ans += f(i)
print(ans)
