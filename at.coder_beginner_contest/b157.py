a = []
for _ in range(3):
    a1, a2, a3  = map(int, input().split())
    a.append([a1, a2, a3])

n = int(input())

b = []
for _ in range(n):
    b1  = map(int, input().split())
    b.append([b1])

for i in range(n):
    for j in range(3):
        for k in range(3):
            if(a[j][k] == b[i]):
                a[j][k] = -1

for i in a:
    print(i)