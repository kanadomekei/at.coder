n = int(input())
c = list(map(int, input().split()))
q = int(input())
ans = 0
for _ in range(q):
    s = list(map(int, input().split()))

    if s[0] == 1:
        x = s[1] - 1 
        a = s[2]
        if c[x] - a >= 0:
            c[x] -= a
            ans += a

    if s[0] == 2:
        a = s[1]
        if len(c[0::2]) == len(list(filter(lambda x : x - a >= 0, c[0::2]))):
            for i in range(0, len(c), 2):
                c[i] -= a
                ans += a
    if s[0] == 3:
        if len(c) == len(list(filter(lambda x : x - a >= 0, c))):
            for i in range(len(c)):
                c[i] -= a          
                ans += a

print(ans)