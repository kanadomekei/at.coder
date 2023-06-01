import math
def f(x1, y1, x2, y2):
    return math.sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2)

x1, y1, x2, y2 = map(int, input().split())

flag = False

for i in range(x1-2, x1+3):
    for j in range(y1-2, y1+3):
        if f(x1, y1, j, i) == f(x2, y2, j, i) == math.sqrt(5):
            flag = True
if flag:
    print('Yes')
else:
    print('No')