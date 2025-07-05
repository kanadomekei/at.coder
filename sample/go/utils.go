package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

// 入力処理
func ReadLine() string {
	scanner := bufio.NewScanner(os.Stdin)
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

func ReadString() string {
	return ReadLine()
}

func ReadStrings() []string {
	line := ReadLine()
	return strings.Fields(line)
}

func Read2DInts(n int) [][]int {
	result := make([][]int, n)
	for i := 0; i < n; i++ {
		result[i] = ReadInts()
	}
	return result
}

// 数学関数
func Max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func Min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func Pow(base, exp int) int {
	result := 1
	for exp > 0 {
		if exp%2 == 1 {
			result *= base
		}
		base *= base
		exp /= 2
	}
	return result
}

func Gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func Lcm(a, b int) int {
	return a / Gcd(a, b) * b
}

// 配列操作
func ReverseInts(arr []int) {
	for i := 0; i < len(arr)/2; i++ {
		arr[i], arr[len(arr)-1-i] = arr[len(arr)-1-i], arr[i]
	}
}

func SumInts(arr []int) int {
	sum := 0
	for _, v := range arr {
		sum += v
	}
	return sum
}

func MaxInts(arr []int) int {
	max := arr[0]
	for _, v := range arr {
		if v > max {
			max = v
		}
	}
	return max
}

func MinInts(arr []int) int {
	min := arr[0]
	for _, v := range arr {
		if v < min {
			min = v
		}
	}
	return min
}

// 素数判定
func IsPrime(n int) bool {
	if n < 2 {
		return false
	}
	if n == 2 {
		return true
	}
	if n%2 == 0 {
		return false
	}
	for i := 3; i*i <= n; i += 2 {
		if n%i == 0 {
			return false
		}
	}
	return true
}

// 順列・組み合わせ
func Factorial(n int) int {
	if n <= 1 {
		return 1
	}
	return n * Factorial(n-1)
}

func Permutation(n, r int) int {
	if r > n || r < 0 {
		return 0
	}
	result := 1
	for i := 0; i < r; i++ {
		result *= (n - i)
	}
	return result
}

func Combination(n, r int) int {
	if r > n || r < 0 {
		return 0
	}
	if r > n-r {
		r = n - r
	}
	result := 1
	for i := 0; i < r; i++ {
		result = result * (n - i) / (i + 1)
	}
	return result
}

// 文字列操作
func ReverseString(s string) string {
	runes := []rune(s)
	for i := 0; i < len(runes)/2; i++ {
		runes[i], runes[len(runes)-1-i] = runes[len(runes)-1-i], runes[i]
	}
	return string(runes)
}

func IsPalindrome(s string) bool {
	runes := []rune(s)
	for i := 0; i < len(runes)/2; i++ {
		if runes[i] != runes[len(runes)-1-i] {
			return false
		}
	}
	return true
}

// ソート
func SortInts(arr []int) {
	sort.Ints(arr)
}

func SortIntsDesc(arr []int) {
	sort.Sort(sort.Reverse(sort.IntSlice(arr)))
}

func SortStrings(arr []string) {
	sort.Strings(arr)
}

// 出力処理
func PrintInts(arr []int) {
	for i, v := range arr {
		if i > 0 {
			fmt.Print(" ")
		}
		fmt.Print(v)
	}
	fmt.Println()
}

func PrintStrings(arr []string) {
	for i, v := range arr {
		if i > 0 {
			fmt.Print(" ")
		}
		fmt.Print(v)
	}
	fmt.Println()
}

// 使用例
func main() {
	// 入力例
	n := ReadInt()
	arr := ReadInts()

	fmt.Printf("Input: n=%d, arr=%v\n", n, arr)
	if len(arr) > 0 {
		fmt.Printf("Max: %d, Min: %d, Sum: %d\n", MaxInts(arr), MinInts(arr), SumInts(arr))
	}
	if len(arr) >= 2 {
		fmt.Printf("GCD of first two: %d\n", Gcd(arr[0], arr[1]))
	}
	fmt.Printf("Is %d prime? %v\n", n, IsPrime(n))

	// Read2DInts example
	fmt.Println("\n--- Read2DInts example ---")
	fmt.Print("Enter number of rows for 2D int array: ")
	n2d := ReadInt()
	if n2d > 0 {
		fmt.Printf("Enter %d lines of space-separated integers:\n", n2d)
		data2d := Read2DInts(n2d)

		fmt.Println("\n2D Int Array:")
		for i, row := range data2d {
			fmt.Printf("Row %d: ", i)
			PrintInts(row)
		}
	}
}
