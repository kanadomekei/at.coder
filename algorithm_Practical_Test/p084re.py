import numpy
a = list(list(map(int,input().split())) for i in range(3))

flag = True
for i in range(3):
    if a[i][0] - a[i][1] != a[i][1] - a[i][2]:
        flag = False
print("Yes" if flag else "No")