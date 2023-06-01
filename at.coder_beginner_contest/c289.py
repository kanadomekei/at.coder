def main():
    N, Q = map(int, input().split())

    # Initialize the boxes
    boxes = [[] for _ in range(N)]

    # Process the queries
    for _ in range(Q):
        query = list(map(int, input().split()))

        if query[0] == 1:
            _, i, j = query
            boxes[j - 1].append(i)
        elif query[0] == 2:
            i = query[1] - 1
            boxes[i].sort()
            print(*boxes[i])
        elif query[0] == 3:
            i = query[1]
            found_boxes = []
            for index, box in enumerate(boxes, start=1):
                if i in box:
                    found_boxes.append(index)
            print(*found_boxes)

if __name__ == "__main__":
    main()
