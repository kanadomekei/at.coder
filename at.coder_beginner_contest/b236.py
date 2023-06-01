n = int(input())
l = list(map(int, input().split()))

sum1 = 0
for i in l:
    sum1 += i

sum2 = n * (n+1) / 2 * 4

print(int(sum2 - sum1))
