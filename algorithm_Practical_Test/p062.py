import collections
s = input()

c = collections.Counter(s)
print(c.most_common()[0][0])