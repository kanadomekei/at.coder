h, w = map(int, input().split())
s = [list(input()) for i in range(h)]

dx = [0, -1, -1, -1, 0, 1, 1, 1]
dy = [1, 1, 0, -1, -1, -1, 0, 1]

ans = [[0]*w for _ in range(h)]
for i in range(h):
    for j in range(w):
        for dx, dy in zip(dx, dy):
            nx = j + dx
            ny = i + dy

            if nx < 0 or nx >= h or ny < 0 or ny >= w:
                continue
            if s[nx][ny] == '#':
                ans[i][j] += 1

print(ans)

