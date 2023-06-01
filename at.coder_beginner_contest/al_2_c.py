n = int(input())

l = []
for _ in range(n):
    l.append(list(input()))


for i in range(n-2, -1, -1): #なんで範囲が－１までなのか？？
    for j in range(2, 2*n-1):
        if(l[i][j] == '#' and (l[i+1][j-1] == 'X' or l[i+1][j] == 'X' or l[i+1][j+1] == 'X')):
            l[i][j] = 'X'

for i in range(0, n):
    l[i] = ''.join(l[i])
    print(l[i])


