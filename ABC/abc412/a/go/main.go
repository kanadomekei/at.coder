package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

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
	n := ReadInt()

	count := 0
	for i := 0; i < n; i++ {
		day := ReadInts()
		target := day[0]
		done := day[1]

		if done > target {
			count++
		}
	}

	fmt.Println(count)
}
