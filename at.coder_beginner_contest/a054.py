a, b = map(int, input().split())

if a != 1 and b != 1 and a > b:
    print('Alice')

elif a != 1 and b != 1 and a < b:
    print('Bob') 

elif a == 1 and b !=1:
      print('Alice')


elif a != 1 and b ==1:
      print('Bob')

else:
    print('Draw')