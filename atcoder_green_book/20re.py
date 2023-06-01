n = int(input())
s = list(input())

q = int(input())

for i in range(q):
    t, a, b = map(int, input().split())

    if t == 1:
        s[a - 1], s[b - 1] = s[b - 1], s[a - 1]
    
    if t == 2:
        s = s[n:2*n] + s[0:n]

print(''.join(s))