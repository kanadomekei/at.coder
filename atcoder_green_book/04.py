n, X = map(int, input().split())
a = list(map(int, input().split()))

a = list(filter(lambda x : x != X, a))

print(*a)