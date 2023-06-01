n = int(input())
a = list(map(int,input().split()))
b = [0,360]
c = 0
for i in a:
    c += i
    c %= 360
    b.append(c)
b.sort()
d = [b[i+1] - b[i] for i in range(len(b)-1)]
print(max(d))
