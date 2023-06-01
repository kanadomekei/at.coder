n, k = map(int, input().split())

def g1(x):
    x = str(x)
    x = list(x)
    x.sort(reverse = True)
    x = "".join(x)
    x = int(x)
    return x 
def g2(x):
    x = str(x)
    x = list(x)
    x.sort()
    x = "".join(x)
    x = int(x)
    return x 

def f(x):
    return g1(x) - g2(x)

a = n
for i in range(k):
    a = f(a)

print(a)