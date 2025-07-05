package main

import (
	"bufio"
	"fmt"
	"os"
	"unicode"
)

func ReadLine() string {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	return scanner.Text()
}

func main() {
	var S, T string
	fmt.Scan(&S, &T)

	tSet := make(map[rune]bool)
	for _, char := range T {
		tSet[char] = true
	}

	sRunes := []rune(S)
	for i := 1; i < len(sRunes); i++ {
		if unicode.IsUpper(sRunes[i]) && !tSet[sRunes[i-1]] {
			fmt.Println("No")
			return
		}
	}

	fmt.Println("Yes")
}
