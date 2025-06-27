package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {
	// ==========================================
	// 制御構文
	// ==========================================

	fmt.Println("=== Goの制御構文 ===")

	// ------------------------------------------
	// if文
	// ------------------------------------------

	fmt.Println("\n=== if文 ===")

	score := 85

	// 基本的なif文
	if score >= 90 {
		fmt.Println("優秀です！")
	} else if score >= 70 {
		fmt.Println("良い成績です。")
	} else if score >= 50 {
		fmt.Println("合格です。")
	} else {
		fmt.Println("がんばりましょう。")
	}

	// 初期化付きif文
	if grade := calculateGrade(score); grade == "A" {
		fmt.Printf("成績は%sです（素晴らしい！）\n", grade)
	} else {
		fmt.Printf("成績は%sです\n", grade)
	}

	// 複雑な条件
	temperature := 25
	isSunny := true

	if temperature > 20 && isSunny {
		fmt.Println("散歩にいい天気です！")
	} else if temperature > 20 || isSunny {
		fmt.Println("まあまあの天気です。")
	} else {
		fmt.Println("家にいましょう。")
	}

	// ------------------------------------------
	// for文
	// ------------------------------------------

	fmt.Println("\n=== for文 ===")

	// 基本的なfor文
	fmt.Print("カウントアップ: ")
	for i := 0; i < 5; i++ {
		fmt.Printf("%d ", i)
	}
	fmt.Println()

	// while文の代わり
	count := 0
	fmt.Print("while風: ")
	for count < 3 {
		fmt.Printf("%d ", count)
		count++
	}
	fmt.Println()

	// 無限ループ（breakで抜ける）
	counter := 0
	fmt.Print("無限ループ: ")
	for {
		if counter >= 3 {
			break
		}
		fmt.Printf("%d ", counter)
		counter++
	}
	fmt.Println()

	// continue文
	fmt.Print("偶数のみ: ")
	for i := 0; i < 10; i++ {
		if i%2 != 0 {
			continue
		}
		fmt.Printf("%d ", i)
	}
	fmt.Println()

	// ------------------------------------------
	// range文
	// ------------------------------------------

	fmt.Println("\n=== range文 ===")

	// スライスに対するrange
	fruits := []string{"apple", "banana", "cherry", "date"}

	fmt.Println("果物リスト:")
	for index, fruit := range fruits {
		fmt.Printf("%d: %s\n", index, fruit)
	}

	// インデックスのみ使用
	fmt.Print("インデックスのみ: ")
	for index := range fruits {
		fmt.Printf("%d ", index)
	}
	fmt.Println()

	// 値のみ使用
	fmt.Print("値のみ: ")
	for _, fruit := range fruits {
		fmt.Printf("%s ", fruit)
	}
	fmt.Println()

	// マップに対するrange
	ages := map[string]int{
		"Alice": 30,
		"Bob":   25,
		"Carol": 35,
	}

	fmt.Println("年齢リスト:")
	for name, age := range ages {
		fmt.Printf("%s: %d歳\n", name, age)
	}

	// 文字列に対するrange（runeとして処理）
	text := "Hello, 世界"
	fmt.Println("文字列の文字:")
	for i, r := range text {
		fmt.Printf("位置 %d: %c (Unicode: U+%04X)\n", i, r, r)
	}

	// ------------------------------------------
	// switch文
	// ------------------------------------------

	fmt.Println("\n=== switch文 ===")

	day := 3

	// 基本的なswitch文
	switch day {
	case 1:
		fmt.Println("月曜日")
	case 2:
		fmt.Println("火曜日")
	case 3:
		fmt.Println("水曜日")
	case 4:
		fmt.Println("木曜日")
	case 5:
		fmt.Println("金曜日")
	case 6, 7: // 複数の値
		fmt.Println("週末")
	default:
		fmt.Println("無効な日付")
	}

	// 初期化付きswitch
	switch dayType := getDayType(day); dayType {
	case "weekday":
		fmt.Println("平日です")
	case "weekend":
		fmt.Println("週末です")
	default:
		fmt.Println("不明な日付タイプ")
	}

	// 条件付きswitch
	switch {
	case score >= 90:
		fmt.Println("A級")
	case score >= 80:
		fmt.Println("B級")
	case score >= 70:
		fmt.Println("C級")
	default:
		fmt.Println("D級")
	}

	// 型switch
	var value interface{} = 42

	switch v := value.(type) {
	case int:
		fmt.Printf("整数: %d\n", v)
	case string:
		fmt.Printf("文字列: %s\n", v)
	case bool:
		fmt.Printf("論理値: %t\n", v)
	default:
		fmt.Printf("不明な型: %T\n", v)
	}

	// fallthroughの使用例
	grade := "B"
	fmt.Print("成績評価: ")
	switch grade {
	case "A":
		fmt.Print("優秀 ")
		fallthrough
	case "B":
		fmt.Print("良好 ")
		fallthrough
	case "C":
		fmt.Print("普通 ")
		fallthrough
	default:
		fmt.Print("以上")
	}
	fmt.Println()

	// ------------------------------------------
	// ラベル付きbreak/continue
	// ------------------------------------------

	fmt.Println("\n=== ラベル付きbreak/continue ===")

	// ネストしたループからの脱出
	fmt.Println("ラベル付きbreak:")
outer:
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			if i == 1 && j == 1 {
				fmt.Println("外側のループを抜けます")
				break outer
			}
			fmt.Printf("(%d, %d) ", i, j)
		}
		fmt.Println()
	}

	// ラベル付きcontinue
	fmt.Println("ラベル付きcontinue:")
outerContinue:
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			if j == 1 {
				fmt.Print("外側の次の反復へ ")
				continue outerContinue
			}
			fmt.Printf("(%d, %d) ", i, j)
		}
		fmt.Println()
	}

	// ------------------------------------------
	// defer文
	// ------------------------------------------

	fmt.Println("\n=== defer文 ===")

	// 基本的なdefer
	fmt.Println("処理開始")
	defer fmt.Println("処理終了（defer）")
	defer fmt.Println("クリーンアップ（defer）")

	fmt.Println("メイン処理")

	// deferは後入れ先出し（LIFO）で実行される
	for i := 0; i < 3; i++ {
		defer fmt.Printf("defer %d\n", i)
	}

	// deferで関数呼び出し
	deferExample()

	// ------------------------------------------
	// select文（基本）
	// ------------------------------------------

	fmt.Println("\n=== select文（基本） ===")

	// チャネルの基本例（詳細は後の章で）
	ch1 := make(chan string, 1)
	ch2 := make(chan string, 1)

	ch1 <- "チャネル1からのメッセージ"

	select {
	case msg1 := <-ch1:
		fmt.Printf("受信: %s\n", msg1)
	case msg2 := <-ch2:
		fmt.Printf("受信: %s\n", msg2)
	default:
		fmt.Println("どのチャネルも準備できていません")
	}

	// タイムアウト付きselect
	timeout := time.After(100 * time.Millisecond)
	select {
	case <-timeout:
		fmt.Println("タイムアウトしました")
	default:
		fmt.Println("すぐに実行されました")
	}

	// ------------------------------------------
	// 実践例：簡単なゲーム
	// ------------------------------------------

	fmt.Println("\n=== 実践例：数当てゲーム ===")

	rand.Seed(time.Now().UnixNano())
	target := rand.Intn(10) + 1
	maxAttempts := 3

	fmt.Printf("1から10の数を当ててください（%d回まで）\n", maxAttempts)

	for attempt := 1; attempt <= maxAttempts; attempt++ {
		// 実際のゲームでは標準入力から読み取りますが、
		// ここでは乱数でシミュレート
		guess := rand.Intn(10) + 1
		fmt.Printf("試行 %d: %d\n", attempt, guess)

		switch {
		case guess == target:
			fmt.Println("正解です！おめでとうございます！")
			return
		case guess < target:
			fmt.Println("もっと大きい数です")
		case guess > target:
			fmt.Println("もっと小さい数です")
		}

		if attempt == maxAttempts {
			fmt.Printf("残念！正解は %d でした\n", target)
		}
	}
}

// ------------------------------------------
// ヘルパー関数
// ------------------------------------------

func calculateGrade(score int) string {
	switch {
	case score >= 90:
		return "A"
	case score >= 80:
		return "B"
	case score >= 70:
		return "C"
	case score >= 60:
		return "D"
	default:
		return "F"
	}
}

func getDayType(day int) string {
	switch day {
	case 1, 2, 3, 4, 5:
		return "weekday"
	case 6, 7:
		return "weekend"
	default:
		return "invalid"
	}
}

func deferExample() {
	fmt.Println("関数開始")
	defer fmt.Println("関数終了（defer）")

	for i := 0; i < 3; i++ {
		defer func(n int) {
			fmt.Printf("ループdefer: %d\n", n)
		}(i) // クロージャで値をキャプチャ
	}

	fmt.Println("関数の主要処理")
}