s,t,x = map(int,input().split())
if (s < t and s <= x < t) or (t < s and (s <= x or x < t)):
  print('Yes')
else:
  print('No')