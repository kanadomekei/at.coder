n = int(input())
a = list(map(int, input().split()))

l = []
for i in range(n):
    l.append(len(set(filter(lambda x : x > a[i], a))))

for i in range(n):
    print(len(filter(list(lambda x : x == i, l))))
    


