s = input()

if s.islower():
    print("No")
elif s.isupper():
    print("No")
elif(len(s) != len(set(s))):
    print('No')
else:
    print('Yes')