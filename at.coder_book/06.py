from re import X


def f(x):
    flag = True
    for i in range(n):
        if x[i] % 2 != 0:
            flag = False
    return flag

n = int(input())
l = list(map(int, input().split()))
ans = 0

while(f(l)):
    ans += 1
    l = list(map(lambda x : x//2, l))

print(ans)
