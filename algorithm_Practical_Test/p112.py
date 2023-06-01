n, m, q = map(int, input().split())

g = [[] for _ in range(n)]
for _ in range(m):
    a, b = map(int, input().split())
    g[a-1].append(b-1)
    g[b-1].append(a-1)  

c = list(map(int, input().split()))

for i in range(q):
    s = list(map(int, input().split()))

    if s[0] == 1:
        x = s[1] - 1
        print(c[x])
        for i  in g[x]:
            c[i] = c[x]
    else:
        x = s[1] - 1
        y = s[2]
        print(c[x])
        c[x] = c[y]

