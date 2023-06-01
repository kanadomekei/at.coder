n, m, q = map(int, input().split())

g = [[] for _ in range(m)]
for _ in range(m):
    a, b = map(int, input().split())
    g[a-1].append(b-1)
    g[b-1].append(a-1)

c = list(map(int, input().split()))

for i in range(q):
    t, x, *y, = map(int, input().split())

    x -= 1

    if t == 1:
        for v in g[x]:
            c[v] = c[x]
    else:
        c[x] = y[0]