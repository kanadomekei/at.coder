s = input()

n = ''

for i in s:
    if i == 'B':
        n = n[:-1]
    else:
        n += i

print(n)