def f(x):
    if x[0] != 'A':
        return False
    if x[2:-1].count('C') != 1:
        return False
    if sum(map(str.islower, x)) != 2:
        return False
    return True

print('AC' if f(input()) else 'WA')

