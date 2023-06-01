from tkinter import N


n, k = map(int, input().split())
a = list(map(int, input().split()))

l = []
for _ in range(n):
    a  = list(map(int, input().split()))
    l.append(a)

print(n)
print(k)

print(a)
print(l)