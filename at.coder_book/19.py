n = int(input())
l = list(map(int, input().split()))

if sum(list(i < 0 for i in l)) % 2 == 0:
    print (sum([abs(i) for i in l]))

else:
    m = min([abs(i) for i in l])
    print (sum([abs(i) for i in l]) - 2 * m)