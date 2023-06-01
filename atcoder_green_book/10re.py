N = int(input())

i = 1
ans = []
while i**2 <= N:
    if N % i == 0:
        ans.append(i)
    if i != N//i:
        ans.append(N//i)
    i += 1

ans.sort()
for i in ans:
    print(i)