a, b, k = map(int, input().split())

ans = 0

for i in range(100):
    if((a * k ** i) >= b):
        ans = i
        break
print(ans)
