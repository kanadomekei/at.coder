x, y = map(int, input().split())
if y <= x:
    print(0)
elif (y - x) % 10 == 0:
    print((y -x ) // 10)
else:
    print((y - x) // 10 + 1)