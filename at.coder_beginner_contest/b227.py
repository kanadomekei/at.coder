N = int(input())
S = list(map(int, input().split()))
MAX = 100
count = 0
for Si in S:
    flg = 0
    for i in range(1, MAX+1):
        for j in range(1, MAX+1):
            if Si == (4*i*j + 3*i + 3*j):
                flg = 1
    if flg == 0:
        count += 1
print(count)