import numpy as np
n = int(input())

a = list(map(int, input().split()))
b = list(map(int, input().split()))

if(np.dot(a, b) == 0):
    print("Yes")
else:
    print('No')