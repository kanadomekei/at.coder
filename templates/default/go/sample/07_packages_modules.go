package main

import (
	"fmt"
	"math"
	"strings"
	"time"
)

func main() {
	fmt.Println("=== Goのパッケージとモジュール ===")

	// 標準ライブラリの使用
	fmt.Println("\n=== 標準ライブラリ ===")

	// math パッケージ
	fmt.Printf("円周率: %.4f\n", math.Pi)
	fmt.Printf("平方根: %.2f\n", math.Sqrt(16))
	fmt.Printf("べき乗: %.0f\n", math.Pow(2, 8))

	// strings パッケージ
	text := "Hello, Go World!"
	fmt.Printf("大文字: %s\n", strings.ToUpper(text))
	fmt.Printf("小文字: %s\n", strings.ToLower(text))
	fmt.Printf("分割: %v\n", strings.Split(text, " "))
	fmt.Printf("置換: %s\n", strings.Replace(text, "Go", "Golang", 1))

	// time パッケージ
	now := time.Now()
	fmt.Printf("現在時刻: %s\n", now.Format("2006-01-02 15:04:05"))
	fmt.Printf("Unix時間: %d\n", now.Unix())

	// 時間の計算
	future := now.Add(24 * time.Hour)
	fmt.Printf("24時間後: %s\n", future.Format("2006-01-02 15:04:05"))

	// 期間の測定
	start := time.Now()
	time.Sleep(100 * time.Millisecond)
	duration := time.Since(start)
	fmt.Printf("処理時間: %v\n", duration)

	// エイリアスの使用
	fmt.Println("\n=== パッケージエイリアス ===")
	
	// import時にエイリアスを設定する例（実際のコードでは import 文で設定）
	// import m "math"
	// fmt.Printf("Sin(π/2): %.2f\n", m.Sin(m.Pi/2))
	
	fmt.Printf("Sin(π/2): %.2f\n", math.Sin(math.Pi/2))

	// 自作の「パッケージ風」関数の使用
	fmt.Println("\n=== カスタム関数 ===")
	
	result := calculator_add(10, 20)
	fmt.Printf("計算結果: %d\n", result)
	
	greeting := formatter_format("Go", "さん")
	fmt.Printf("フォーマット結果: %s\n", greeting)
}

// 自作の「パッケージ風」関数群
// 実際のプロジェクトでは別ファイルや別パッケージに分割する

// calculator パッケージ風の関数
func calculator_add(a, b int) int {
	return a + b
}

func calculator_multiply(a, b int) int {
	return a * b
}

// formatter パッケージ風の関数
func formatter_format(name, suffix string) string {
	return fmt.Sprintf("こんにちは、%s%s！", name, suffix)
}

func formatter_formatWithTime(name string) string {
	now := time.Now()
	hour := now.Hour()
	
	var greeting string
	switch {
	case hour < 12:
		greeting = "おはよう"
	case hour < 18:
		greeting = "こんにちは"
	default:
		greeting = "こんばんは"
	}
	
	return fmt.Sprintf("%s、%sさん！", greeting, name)
}

// 初期化関数のシミュレーション
func init() {
	fmt.Println("パッケージが初期化されました")
}