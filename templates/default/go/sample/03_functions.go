package main

import (
	"errors"
	"fmt"
	"math"
	"strings"
)

func main() {
	fmt.Println("=== Goの関数 ===")

	// 基本的な関数呼び出し
	greet()
	greetPerson("Alice")

	// 返り値のある関数
	sum := add(10, 20)
	fmt.Printf("10 + 20 = %d\n", sum)

	// 複数の返り値
	quotient, remainder := divideWithRemainder(17, 5)
	fmt.Printf("17 ÷ 5 = %d 余り %d\n", quotient, remainder)

	// 名前付き返り値
	result := calculateStats([]int{1, 2, 3, 4, 5})
	fmt.Printf("統計: 合計=%d, 平均=%.2f, 最大=%d\n", result.sum, result.avg, result.max)

	// 可変長引数
	total := addAll(1, 2, 3, 4, 5)
	fmt.Printf("可変長引数の合計: %d\n", total)

	numbers := []int{10, 20, 30}
	total2 := addAll(numbers...) // スライスを展開
	fmt.Printf("スライス展開: %d\n", total2)

	// 関数を変数に代入
	operation := add
	result2 := operation(5, 3)
	fmt.Printf("関数変数: %d\n", result2)

	// 高階関数
	result3 := applyOperation(10, 5, add)
	result4 := applyOperation(10, 5, multiply)
	fmt.Printf("高階関数: %d, %d\n", result3, result4)

	// 匿名関数
	square := func(x int) int {
		return x * x
	}
	fmt.Printf("匿名関数: %d\n", square(5))

	// クロージャ
	increment := makeIncrementer(10)
	fmt.Printf("クロージャ: %d\n", increment())
	fmt.Printf("クロージャ: %d\n", increment())

	// 関数型の宣言と使用
	var calc Calculator = add
	fmt.Printf("関数型: %d\n", calc(3, 7))

	// エラーを返す関数
	result5, err := safeDivide(10, 2)
	if err != nil {
		fmt.Printf("エラー: %v\n", err)
	} else {
		fmt.Printf("除算結果: %.2f\n", result5)
	}

	result6, err := safeDivide(10, 0)
	if err != nil {
		fmt.Printf("エラー: %v\n", err)
	} else {
		fmt.Printf("除算結果: %.2f\n", result6)
	}

	// 再帰関数
	fmt.Printf("階乗 5! = %d\n", factorial(5))
	fmt.Printf("フィボナッチ 10 = %d\n", fibonacci(10))

	// メソッド
	rect := Rectangle{width: 10, height: 5}
	fmt.Printf("長方形の面積: %d\n", rect.Area())
	fmt.Printf("長方形の周囲: %d\n", rect.Perimeter())

	rect.Scale(2)
	fmt.Printf("拡大後の面積: %d\n", rect.Area())

	// ポインタレシーバーとバリューレシーバーの違い
	p := Point{x: 3, y: 4}
	fmt.Printf("距離（値レシーバー）: %.2f\n", p.Distance())

	p.Move(1, 1) // ポインタレシーバー
	fmt.Printf("移動後の点: (%d, %d)\n", p.x, p.y)
}

// 基本的な関数
func greet() {
	fmt.Println("こんにちは！")
}

func greetPerson(name string) {
	fmt.Printf("こんにちは、%sさん！\n", name)
}

func add(a, b int) int {
	return a + b
}

func multiply(a, b int) int {
	return a * b
}

// 複数の返り値
func divideWithRemainder(dividend, divisor int) (int, int) {
	return dividend / divisor, dividend % divisor
}

// 名前付き返り値
type Stats struct {
	sum int
	avg float64
	max int
}

func calculateStats(numbers []int) (result Stats) {
	if len(numbers) == 0 {
		return
	}

	result.sum = 0
	result.max = numbers[0]

	for _, num := range numbers {
		result.sum += num
		if num > result.max {
			result.max = num
		}
	}

	result.avg = float64(result.sum) / float64(len(numbers))
	return // 名前付き返り値なので値を指定しなくても良い
}

// 可変長引数
func addAll(numbers ...int) int {
	total := 0
	for _, num := range numbers {
		total += num
	}
	return total
}

// 高階関数
func applyOperation(a, b int, op func(int, int) int) int {
	return op(a, b)
}

// クロージャを返す関数
func makeIncrementer(start int) func() int {
	count := start
	return func() int {
		count++
		return count
	}
}

// 関数型の定義
type Calculator func(int, int) int

// エラーを返す関数
func safeDivide(a, b float64) (float64, error) {
	if b == 0 {
		return 0, errors.New("ゼロで割ることはできません")
	}
	return a / b, nil
}

// 再帰関数
func factorial(n int) int {
	if n <= 1 {
		return 1
	}
	return n * factorial(n-1)
}

func fibonacci(n int) int {
	if n <= 1 {
		return n
	}
	return fibonacci(n-1) + fibonacci(n-2)
}

// 構造体とメソッド
type Rectangle struct {
	width, height int
}

// 値レシーバーのメソッド
func (r Rectangle) Area() int {
	return r.width * r.height
}

func (r Rectangle) Perimeter() int {
	return 2 * (r.width + r.height)
}

// ポインタレシーバーのメソッド
func (r *Rectangle) Scale(factor int) {
	r.width *= factor
	r.height *= factor
}

type Point struct {
	x, y int
}

// 値レシーバー（構造体のコピーで動作）
func (p Point) Distance() float64 {
	return math.Sqrt(float64(p.x*p.x + p.y*p.y))
}

// ポインタレシーバー（元の構造体を変更）
func (p *Point) Move(dx, dy int) {
	p.x += dx
	p.y += dy
}

// インターフェースの例
type Stringer interface {
	String() string
}

func (r Rectangle) String() string {
	return fmt.Sprintf("Rectangle{width: %d, height: %d}", r.width, r.height)
}

func (p Point) String() string {
	return fmt.Sprintf("Point{x: %d, y: %d}", p.x, p.y)
}

// ジェネリック風の処理（interface{}を使用）
func processValues(values ...interface{}) {
	for i, value := range values {
		switch v := value.(type) {
		case int:
			fmt.Printf("値 %d: 整数 %d\n", i, v)
		case string:
			fmt.Printf("値 %d: 文字列 %s\n", i, v)
		case Stringer:
			fmt.Printf("値 %d: Stringer %s\n", i, v.String())
		default:
			fmt.Printf("値 %d: 不明な型 %T\n", i, v)
		}
	}
}

// 関数リテラルとクロージャの実践例
func createValidator(minLength int) func(string) bool {
	return func(s string) bool {
		return len(strings.TrimSpace(s)) >= minLength
	}
}

func createFormatter(prefix, suffix string) func(string) string {
	return func(s string) string {
		return prefix + s + suffix
	}
}