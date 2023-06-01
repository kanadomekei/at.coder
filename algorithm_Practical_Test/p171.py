import collections
def f(n):
    if collections.Counter(str(n)) == {'3': 1, '5': 1, '7': 1}:
        return 1
    else:
        return 0
ans = 0
for i in range(1, int(input())):
    ans += f(i)

print(ans)
