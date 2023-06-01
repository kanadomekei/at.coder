k = int(input())
a, b = map(int, input().split())

flag = False

for i in range(a, b+1):
    if i % 7 == 0:
        flag = True

print('Ok' if flag else 'No')

