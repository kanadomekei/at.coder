def rotate(matrix):
    return list(zip(*matrix[::-1]))

def main():
    N = int(input())
    A = [list(map(int, input().split())) for _ in range(N)]
    B = [list(map(int, input().split())) for _ in range(N)]

    for _ in range(4):
        if A == B:
            print("Yes")
            return
        A = rotate(A)

    print("No")

if __name__ == "__main__":
    main()
