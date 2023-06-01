from itertools import  permutations

n, k = map(int, input().split())
l = [list(map(int, input().split())) for _ in range(n)]

ans = 0
for root in list(permutations(range(1, n))):
    sum = 0
    for i in root:
        i -= 1
        sum += [i][i + 1] 
        if sum == k:
            ans += 1

print(ans)
