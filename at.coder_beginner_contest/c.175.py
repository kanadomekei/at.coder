x, k, d = map(int, input().split())

x = abs(x)

if x - d*k >= 0:
    print(abs(x - d*k))
else:
    move_count = k - x // d
    
    jump_before=x-d*(x//d)
    jump_after=jump_before-d

    if move_count%2==0:
        print(abs(jump_before))
    else:
        print(abs(jump_after))