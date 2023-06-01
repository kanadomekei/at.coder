import math

n, w = map(int, input().split())
a = list(map(int, input().split()))


def combinations_count(n, r):
    return math.factorial(n) // (math.factorial(n - r) * math.factorial(r))

cnt = 0
k =1

for i in range(1<<len(a)):
    l = []
    for j in range(len(a)):
        if (i>>j & 1) == 1:
            l.append(a[j])
    if sum(l) == k:
        cnt += 1
print('Yes' if cnt>=1 else 'No')
