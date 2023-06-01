s = input()
l = []
ans = ''
i = 0
while i < len(s):
    j = i + 1
    while j < len(s) and s[j].islower():
        j += 1
    l.append(s[i: j+1])
    i = j + 1
print(ans.join(sorted(l, key = str.lower)))


