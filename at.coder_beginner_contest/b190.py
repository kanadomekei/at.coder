N,S,D = map(int,input().split())

for _ in range(N):
    x,y = map(int,input().split())
    if x < S and y > D: exit(print('Yes'))

print('No')