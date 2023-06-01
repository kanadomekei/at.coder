h, w, x, y = map(int, input().split())

grid = []

for _ in range(h):
    s = list(input())
    grid.append(s)

x -= 1
y -= 1

