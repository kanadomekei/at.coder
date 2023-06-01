from unicodedata import name


n = set(input())
count = 0
i = 0
while(True):
    if(len(set(count)) == 1):
        count += 1
    if(count == n):
        break
    i += 1
    

