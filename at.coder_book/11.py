n, y = map(int, input().split())

for i in range(n + 1):
    for j in range(n + 1):
        k = n - i - j
        if k < 0 or n < k:
            continue
        if 10000 * i + 5000 * j + 1000 * k == y:
            print(i, j, k)
            