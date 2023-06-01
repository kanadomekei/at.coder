k = int(input())
s, t = map(int, input().split())

flag = False
for i in range(s,t+1):
    if i % k == 0:
        flag = True

print("Ok" if flag else "NG")

