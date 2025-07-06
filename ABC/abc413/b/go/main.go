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
	s := make([]string, n)
	for i := 0; i < n; i++ {
		s[i] = ReadLine()
	}

	uniqueStrings := make(map[string]bool)

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if i == j {
				continue
			}
			concatenated := s[i] + s[j]
			uniqueStrings[concatenated] = true
		}
	}

	fmt.Println(len(uniqueStrings))
}
