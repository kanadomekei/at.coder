n = int(input())
s = input()
q = input()

l = []
for _ in range(n):
    t, a, b  = map(int, input().split())
    l.append([t, a, b])

for i in range(n):
    #if(l[i][0] = 1):
        s[l[i][1]], s[l[i][2]] = s[l[i][2]], s[l[i][1]]
    #else:
        s[0:n+1], s[n+1:2*n+1] = s[n+1:2*n+1], s[0:2*n+1]

print("".join(s))