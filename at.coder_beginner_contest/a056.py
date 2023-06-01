a, b = map(str, input().split())
if a == 'H' and b == 'H':
    print('H')
elif a == 'H' and b == 'D':
    print('D')
elif a == 'D' and b == 'D':
    print('H')
else:
    print('D')
