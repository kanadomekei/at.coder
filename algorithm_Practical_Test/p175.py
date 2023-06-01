n = int(input())
h = list(map(int, input().split()))

cost = [0, abs(h[1] - h[0])] + [-1] * (n - 2)

for i in range(2, n):
    cost[i] = min(cost[i - 1] + abs(h[i] - h[i-1]), cost[i - 2] + abs(h[i] - h[i-2]))

print(cost[-1])

