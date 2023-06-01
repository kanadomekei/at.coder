from collections import deque

r, c = map(int, input().split())
sy, sx = map(int, input().split())
gy, gx = map(int, input().split())

l = [list(input()) for _ in range(c)]

sy, sx, gy, gx = sy - 1, sx - 1, gy - 1, gx - 1


dist = [[-1 for j in range(c)] for i in range(r)]

#キューを用意して、始点を入れる
q = deque()
q.append([sy, sx])
dist[sy][sx] = 0

#キューから取り出して探索する
while len(q) > 0:
    i, j = q.popleft()
    #4つの隣マスを確認する

    for ni, nj in [[i+1, j], [i-1, j], [i, j+1], [i, j-1]]:

        if not(0 <= ni < r and 0 <= nj < c):
            continue
        if l[i][j] == '#':
            continue
        if dist[ni][nj] == -1:
            dist[ni][nj] = dist[i][j] + 1
            q.append([ni,nj])

print(dist[gy][gx])

