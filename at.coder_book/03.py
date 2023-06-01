from re import T


s, t = map(int, input().split())
print(s//t if s%t == 0 else s//t+1)
