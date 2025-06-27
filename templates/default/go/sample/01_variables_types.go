package main

import (
	"fmt"
	"math"
	"reflect"
)

func main() {
	// ==========================================
	// 変数とデータ型
	// ==========================================

	fmt.Println("=== Goの変数とデータ型 ===")

	// ------------------------------------------
	// 変数の宣言
	// ------------------------------------------

	fmt.Println("\n=== 変数の宣言 ===")

	// var による宣言
	var name string
	name = "Alice"
	fmt.Printf("名前: %s\n", name)

	// 初期化付き宣言
	var age int = 30
	var height float64 = 165.5
	fmt.Printf("年齢: %d, 身長: %.1f\n", age, height)

	// 型推論
	var active = true // bool型として推論
	fmt.Printf("アクティブ: %t\n", active)

	// 短縮変数宣言（:= 演算子）
	email := "alice@example.com"
	fmt.Printf("メール: %s\n", email)

	// 複数変数の同時宣言
	var x, y int = 10, 20
	fmt.Printf("x: %d, y: %d\n", x, y)

	// 異なる型の複数変数
	var (
		username    string  = "bob"
		userAge     int     = 25
		userBalance float64 = 1000.50
	)
	fmt.Printf("ユーザー: %s, 年齢: %d, 残高: %.2f\n", username, userAge, userBalance)

	// ------------------------------------------
	// 基本的なデータ型
	// ------------------------------------------

	fmt.Println("\n=== 基本的なデータ型 ===")

	// 整数型
	var int8Val int8 = 127           // -128 to 127
	var int16Val int16 = 32767       // -32768 to 32767
	var int32Val int32 = 2147483647  // -2147483648 to 2147483647
	var int64Val int64 = 9223372036854775807
	var intVal int = 42              // プラットフォーム依存（32または64bit）

	fmt.Printf("符号付き整数:\n")
	fmt.Printf("int8: %d, int16: %d, int32: %d\n", int8Val, int16Val, int32Val)
	fmt.Printf("int64: %d, int: %d\n", int64Val, intVal)

	// 符号なし整数
	var uint8Val uint8 = 255                    // 0 to 255
	var uint16Val uint16 = 65535                // 0 to 65535
	var uint32Val uint32 = 4294967295           // 0 to 4294967295
	var uint64Val uint64 = 18446744073709551615
	var uintVal uint = 100                      // プラットフォーム依存

	fmt.Printf("符号なし整数:\n")
	fmt.Printf("uint8: %d, uint16: %d, uint32: %d\n", uint8Val, uint16Val, uint32Val)
	fmt.Printf("uint64: %d, uint: %d\n", uint64Val, uintVal)

	// 特別な整数型
	var byteVal byte = 255      // uint8のエイリアス
	var runeVal rune = 'A'      // int32のエイリアス（Unicode文字）
	var uintptrVal uintptr = 0x1000 // ポインタのサイズ

	fmt.Printf("特別な型: byte: %d, rune: %c (%d), uintptr: 0x%x\n", 
		byteVal, runeVal, runeVal, uintptrVal)

	// ------------------------------------------
	// 浮動小数点型
	// ------------------------------------------

	fmt.Println("\n=== 浮動小数点型 ===")

	var float32Val float32 = 3.14159      // 32bit浮動小数点
	var float64Val float64 = 2.718281828459045 // 64bit浮動小数点

	fmt.Printf("float32: %f, float64: %f\n", float32Val, float64Val)

	// 科学記法
	var scientific1 = 1e6    // 1,000,000
	var scientific2 = 2.5e-3 // 0.0025

	fmt.Printf("科学記法: 1e6 = %g, 2.5e-3 = %g\n", scientific1, scientific2)

	// 特別な浮動小数点値
	fmt.Printf("無限大: %f\n", math.Inf(1))
	fmt.Printf("NaN: %f\n", math.NaN())

	// ------------------------------------------
	// 複素数型
	// ------------------------------------------

	fmt.Println("\n=== 複素数型 ===")

	var complex64Val complex64 = 1 + 2i      // 32bit実部+32bit虚部
	var complex128Val complex128 = 3 + 4i    // 64bit実部+64bit虚部

	fmt.Printf("complex64: %v\n", complex64Val)
	fmt.Printf("complex128: %v\n", complex128Val)

	// 複素数の操作
	fmt.Printf("実部: %g, 虚部: %g\n", real(complex128Val), imag(complex128Val))
	fmt.Printf("絶対値: %g\n", cmplx.Abs(complex128Val))

	// ------------------------------------------
	// 論理型
	// ------------------------------------------

	fmt.Println("\n=== 論理型 ===")

	var isGoAwesome bool = true
	var isLearning bool = false

	fmt.Printf("Goは素晴らしい: %t\n", isGoAwesome)
	fmt.Printf("学習中: %t\n", isLearning)

	// 論理演算
	andResult := isGoAwesome && isLearning
	orResult := isGoAwesome || isLearning
	notResult := !isGoAwesome

	fmt.Printf("AND: %t, OR: %t, NOT: %t\n", andResult, orResult, notResult)

	// ------------------------------------------
	// 文字列型
	// ------------------------------------------

	fmt.Println("\n=== 文字列型 ===")

	var greeting string = "Hello, Go!"
	var multiline string = `これは
複数行の
文字列です`

	fmt.Printf("挨拶: %s\n", greeting)
	fmt.Printf("複数行:\n%s\n", multiline)

	// 文字列の操作
	fmt.Printf("文字列の長さ: %d\n", len(greeting))
	fmt.Printf("最初の文字: %c\n", greeting[0])

	// 文字列の連結
	fullGreeting := greeting + " " + "Welcome!"
	fmt.Printf("連結された文字列: %s\n", fullGreeting)

	// rune（Unicode文字）
	japanese := "こんにちは"
	fmt.Printf("日本語文字列: %s\n", japanese)
	fmt.Printf("バイト長: %d\n", len(japanese))

	// runeとして文字を処理
	for i, r := range japanese {
		fmt.Printf("位置 %d: %c (Unicode: U+%04X)\n", i, r, r)
	}

	// ------------------------------------------
	// 定数
	// ------------------------------------------

	fmt.Println("\n=== 定数 ===")

	const Pi = 3.14159
	const MaxUsers = 1000
	const AppName = "MyGoApp"

	fmt.Printf("円周率: %g\n", Pi)
	fmt.Printf("最大ユーザー数: %d\n", MaxUsers)
	fmt.Printf("アプリ名: %s\n", AppName)

	// 定数グループ
	const (
		StatusOK       = 200
		StatusNotFound = 404
		StatusError    = 500
	)

	fmt.Printf("ステータス: %d, %d, %d\n", StatusOK, StatusNotFound, StatusError)

	// iota を使った定数
	const (
		Monday = iota // 0
		Tuesday       // 1
		Wednesday     // 2
		Thursday      // 3
		Friday        // 4
		Saturday      // 5
		Sunday        // 6
	)

	fmt.Printf("曜日: 月=%d, 火=%d, 水=%d\n", Monday, Tuesday, Wednesday)

	// ------------------------------------------
	// 型変換
	// ------------------------------------------

	fmt.Println("\n=== 型変換 ===")

	var i int = 42
	var f float64 = float64(i)
	var u uint = uint(f)

	fmt.Printf("int -> float64 -> uint: %d -> %g -> %d\n", i, f, u)

	// 文字列と数値の変換
	import "strconv"

	// 数値から文字列
	numStr := strconv.Itoa(123)
	fmt.Printf("数値から文字列: %s\n", numStr)

	// 文字列から数値
	str := "456"
	num, err := strconv.Atoi(str)
	if err == nil {
		fmt.Printf("文字列から数値: %d\n", num)
	}

	// 浮動小数点の変換
	floatStr := strconv.FormatFloat(3.14159, 'f', 2, 64)
	fmt.Printf("浮動小数点から文字列: %s\n", floatStr)

	parsedFloat, err := strconv.ParseFloat("2.718", 64)
	if err == nil {
		fmt.Printf("文字列から浮動小数点: %g\n", parsedFloat)
	}

	// ------------------------------------------
	// ゼロ値
	// ------------------------------------------

	fmt.Println("\n=== ゼロ値 ===")

	var zeroInt int
	var zeroFloat float64
	var zeroBool bool
	var zeroString string
	var zeroSlice []int

	fmt.Printf("ゼロ値:\n")
	fmt.Printf("int: %d\n", zeroInt)
	fmt.Printf("float64: %g\n", zeroFloat)
	fmt.Printf("bool: %t\n", zeroBool)
	fmt.Printf("string: '%s'\n", zeroString)
	fmt.Printf("slice: %v (nil: %t)\n", zeroSlice, zeroSlice == nil)

	// ------------------------------------------
	// 型の確認
	// ------------------------------------------

	fmt.Println("\n=== 型の確認 ===")

	var value interface{} = 42

	fmt.Printf("値: %v, 型: %T\n", value, value)
	fmt.Printf("reflectによる型: %v\n", reflect.TypeOf(value))

	// 型アサーション
	if intValue, ok := value.(int); ok {
		fmt.Printf("int型として取得: %d\n", intValue)
	}

	// 型switch
	switch v := value.(type) {
	case int:
		fmt.Printf("整数です: %d\n", v)
	case string:
		fmt.Printf("文字列です: %s\n", v)
	case bool:
		fmt.Printf("論理値です: %t\n", v)
	default:
		fmt.Printf("不明な型です: %T\n", v)
	}
}