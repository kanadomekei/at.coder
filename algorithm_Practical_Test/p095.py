from unittest import result


n = int(input())

i = 0
result = 0
while True:
    i += 1
    i = str(i)
    if len(set(i)) == 1:
        result += 1
    if result == n:
        print(int(i))
        break
    i = int(i)

    

