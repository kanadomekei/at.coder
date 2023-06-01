n, s, d = map(int, input().split())
flag = False
for _ in range(n):
    x, y = map(int, input().split())
    if x < s and d < y:
        flag = True

print("Yes" if flag else "No")