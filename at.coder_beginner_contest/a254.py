n = int(input())
if(n % 100 >= 10):
    print(n % 100)
else:
    print('0' + str(n % 100))

