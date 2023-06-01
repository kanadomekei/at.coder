n = int(input())

if n >= 100:
    print('AGC' + str(n+1))
elif n >= 42:
    print('AGC0' + str(n+1))
elif n >= 10:
    print('AGC0' + str(n))
else:
    print('AGC00' + str(n))