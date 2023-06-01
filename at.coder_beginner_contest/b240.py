n = int(input())

l = list(map(int, input().split()))

l = sorted(l)

sum = 0
flag = 0

for i in range(0, n):
    if flag != l[i]:
        sum += 1
        flag = l[i]

print(sum)
