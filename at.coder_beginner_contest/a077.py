a = input()
b = input()

if a[0] == b[-1] and a[1] == b[1] and a[-1] == b[0]:
    print('YES')
else:
    print('NO')