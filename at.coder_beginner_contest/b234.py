import math
def f(x1, y1, x2, y2):
    return math.sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2)
    
n = int(input())
a = [list(map(int,input().split())) for i in range(n)]

max = 0

for i in a:
    for j in a:
        if max < f(*i, *j):
            max = f(*i, *j)
print(max)