n, q = map(int, input().split())
s = input()
ans = [] * len(s)
i = 0
while i < len(n):
    j = i + 1
    if s[i, j + 1] == 'AC':
        ans[j] == 1 
    



for _ in range(q):
    l, r = map(int, input().split())
    print(sum(ans[l:r]))
