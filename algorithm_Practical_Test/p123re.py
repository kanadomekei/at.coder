from unittest import skip


n, q = map(int, input().split())

l = [[False for j in range(n)] for i in range(n)]

for _ in range(q):
    s = list(map(int, input().split()))
        
    a = s[1] - 1

    if s[0] == 1:
        b = s[2] - 1
        l[a][b] = True
    if s[0] == 2:

        for i in range(n):
            if l[i][a]:
                l[a][i] = True

    if s[0] == 3:
        to_follow = []

        for i in range(n):
            if l[a][i]:
                for j in range(n):
                    if l[i][j] and j != a:
                        to_follow.append(j)
                        
        for j in to_follow:
            l[a][j] = True
print(*l)
    

