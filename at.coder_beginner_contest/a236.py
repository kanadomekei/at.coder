S = input()
a = int(input())
b = int(input())

x = S[a]
S[a] = S[b]
S[b] = x

print(S)