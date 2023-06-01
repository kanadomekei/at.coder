h, w, rs, cs = map(int, input().split())
n = int(input())
maze =  [['.' for j in range(n)] for i in range(n)]

for _ in range(n):
    r, c = map(int, input().split())
    r -= 1
    c -= 1

    maze[r][c] = '#'


q = int(input())

rs -= 1
cs -= 1

for i in range(q):
    d, l = map(int, input().split())

    if d == 'L' and 0 <= cs < n and maze[rs][cs - 1] != '#':
        print()

    if d == 'R' and 0 <= cs < n and maze[rs][cs + 1] != '#':
        print()
    
    if d == 'U' and 0 <= rs < n and maze[rs - 1][cs] != '#':
        print()
    
    if d == 'D' and 0 <= rs < n and maze[rs + 1][cs] != '#':
        print()