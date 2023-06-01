h, w, x, y = map(int, input().split())

l = [list(input()) for _ in range(h)]

ans = 0

x -= 1
y -= 1

# 上
# 上
for i in range(1,h):
    if 0<=x-i<h:
        if l[x-i][y]=="#":
            break
        else:
            ans+=1
# 下
for i in range(1,h):
    if 0<=x-i<h:
        if l[x+i][y]=="#":
            break
        else:
            ans+=1
# 左
for i in range(1,w):
    if 0<=y-i<h:
        if l[x][y-i]=="#":
            break
        else:
            ans+=1
# 右
for i in range(1,w):
    if 0<=y+i<h:
        if l[x][y+1]=="#":
            break
        else:
            ans+=1
if l[x][y] == '.':
    ans += 1

print(ans)

