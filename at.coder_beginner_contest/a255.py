r, c = map(int, input().split())

l = []
for _ in range(2):
    a  = list(map(int, input().split()))
    l.append(a)

print(l[r-1][c-1])


