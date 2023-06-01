def f(x):
    s = sum(list(map(int, list(str(x)))))
    count_1= len(list(filter(lambda x : x % 3 == 1, map(int, list(str(x))))))
    count_2= len(list(filter(lambda x : x % 3 == 2, map(int, list(str(x))))))

    if s < 3:
        return -1

    if s % 3 == 0:
        return 0
    
    if s % 3 == 1 :
        if count_1 >= 1 and len(str(x)) > 1:
            return 1
        if count_2 >= 2 and len(str(x)) > 2:
            return 2
    
    if s % 3 == 2:
        if count_2 >= 1 and len(str(x)) > 1:
            return 1    
        if count_1 >= 2 and len(str(x)) > 2:
            return 2

    return -1


n = int(input())
print(f(n))


