a, b, x = map(int, input().split())

sum = 0
for i in range(a, b+1):
    if i % x == 0:
        sum += 1 

print(sum)

