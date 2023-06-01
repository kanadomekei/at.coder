def f(x1, y1, x2, y2, x3, y3):
    if (x2 - x1)*(y3 - y1) == (y2 - y1)*(x3 - x1):
        return True
    return False

n = int(input())
l = []
for _ in range(n):
    s, t = map(int, input().split())
    l.append([s, t])

flag = False
for i in range(n):
    for j in range(i+1, n):
        for k in range(j+1, n):
            if f(l[i][0], l[i][1], l[j][0], l[j][1], l[k][0], l[k][1]):
                flag = True

print("Yes" if flag else "No")
                
