def f():
    n = int(input())
    pt, px, py = 0, 0, 0

    for _ in range(n):
        t, x, y = map(int, input().split())

        t, x, y = t - pt, abs(x - px), abs(y - py)

        if t < x + y:
            return False
        
        if t % 2 != (x + y) % 2:
            return False
        
        pt, px, py = t, x, y
    return True

print('Yes' if f() else 'No')
