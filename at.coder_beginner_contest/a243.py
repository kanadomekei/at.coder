v, a, b, c = map(int, input().split())

while(True):
    if(v - a >= 0):
        v -= a 
    else:
        print('F')
        break
    if(v - b >= 0):
        v -= b 
    else:
        print('M')
        break
    if(v - c >= 0):
        v -= c
    else:
        print('T')
        break