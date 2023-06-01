l, r = map(int, input().split())
s = input()
a = s[l-1:r]
s = s[0:l-1] + a[::-1] + s[r:]

print(s)