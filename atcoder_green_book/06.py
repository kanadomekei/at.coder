n = int(input())
l = []
for _ in range(n):
    s, t = map(str, input().split())
    t = int(t)
    l.append([t, s])

l.sort(reverse=True)
print(l[1][1])



