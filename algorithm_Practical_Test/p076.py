a = list(list(map(int,input().split())) for i in range(3))
n = int(input())

l = [[False] * 3, [False] * 3, [False] * 3]

for i in range(n):
    b = int(input())
    for j in range(3):
        for k in range(3):
            if a[j][k] == b:
                l[j][k] = True

flag = False
for i in range(3):
    if l[i][:] == [True] * 3 or l[:][i] == [True] * 3:
        flag = True
    if l[i][i] == True and l[i][i] == True and l[i][i] == True:
        flag = True
    if l[2-i][2-i] == True and l[2-i][2-i] == True and l[2-i][2-i] == True:
        flag = True

print('Yes' if flag else 'No')