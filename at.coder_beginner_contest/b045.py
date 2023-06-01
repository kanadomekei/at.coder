a = input()
b = input()
c = input()
d ={'a':a, 'b':b, 'c':c}
turn = 'a'
while True:
    if len(d[turn]) == 0:
        ans = turn.upper()
        break
    else:
        nxt = d[turn][0]
        d[turn] = d[turn][1:]
        turn = nxt

print(ans)