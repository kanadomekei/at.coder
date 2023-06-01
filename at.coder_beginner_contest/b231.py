from collections import Counter
n = int(input())
l = [input() for i in range(n)]
a = Counter(l).most_common()

print(a[0][0])