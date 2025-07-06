package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Pair struct {
	X int64
	C int64
}

var scanner = bufio.NewScanner(os.Stdin)

func ReadLine() string {
	scanner.Scan()
	return scanner.Text()
}

func ReadInt() int {
	line := ReadLine()
	n, _ := strconv.Atoi(line)
	return n
}

func ReadInts() []int {
	line := ReadLine()
	strs := strings.Fields(line)
	ints := make([]int, len(strs))
	for i, str := range strs {
		ints[i], _ = strconv.Atoi(str)
	}
	return ints
}

func main() {
	q := ReadInt()

	list := []Pair{}
	head := 0

	for i := 0; i < q; i++ {
		query := ReadInts()
		if query[0] == 1 {
			list = append(list, Pair{X: int64(query[2]), C: int64(query[1])})
		} else {
			k := int64(query[1])
			var sum int64 = 0

			for k > 0 && head < len(list) {
				if list[head].C <= k {
					sum += list[head].X * list[head].C
					k -= list[head].C
					head++
				} else {
					sum += list[head].X * k
					list[head].C -= k
					k = 0
				}
			}
			fmt.Println(sum)
		}
	}

}
