N, W = map(int,input().split())
A = [list(map(int,input().split())) for i in range(N)]

for i in zip(*A):
    print(*i)


