import math
x, a, b = map(int, input().split())

if math.fabs(x - a) > math.fabs(x - b):
    print('B')
else:
    print('A')
