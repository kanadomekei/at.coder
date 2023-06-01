a, b, c, x = map(int, input().split())

if(x <= a):
    print(1.000000000000)
elif(a <= x <= b):
    print(c / (b - a))
else:
    print(0.000000000000)