w = input()

l = sorted(set(w))

flag = True

for i in l:
   if w.count(i) % 2 == 1:
     flag = False

if flag:
    print('Yes')
else:
    print('No')