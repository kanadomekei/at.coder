import sys
sys.setrecursionlimit(10000000)
h, w = map(int, input().split())

s = [list(input()) for _ in range(h)]

for i in range(h):
    for j in range(w):
        if s[i][j] == 's':
            si, sj = i, j
        if s[i][j] == 'g':
            gi, gj = i, j

visited =  [[False for j in range(w)] for i in range(h)]

def dfs(i, j): #スタート地点を引数にする
    visited[i][j] = True
    #4方向の隣りマスを探索する
    for ni, nj in [[i+1, j], [i-1, j], [i, j+1], [i, j-1]]:
        #もし盤面の範囲内でなければ無視する
        if not( 0 <= ni < h and 0 <= nj < w):
            continue
        #もし壁マスであれば無視する
        if s[i][j] == '#':
            continue
        #もし未訪問であれば再帰的に呼び出す
        if not visited[ni][nj]:
            dfs(ni, nj)
dfs(si, sj)
print('Yes' if visited[gi][gj] else 'No')

