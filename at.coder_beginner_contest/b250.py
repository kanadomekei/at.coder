n, a, b = map(int, input().split())

list = []

for _ in range(a):
    list.append(['.'] * b)
    
for _ in range(a):
    list.append(['#'] * b)

print(list)