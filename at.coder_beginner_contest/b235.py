n = int(input())
l = list(map(int, input().split()))

h = 0

for i in l:
    if h < i :
        h = i
    else:
        break

print(h)



