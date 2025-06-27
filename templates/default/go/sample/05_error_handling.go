package main

import (
	"errors"
	"fmt"
	"strconv"
)

func main() {
	fmt.Println("=== Goのエラーハンドリング ===")

	// 基本的なエラーハンドリング
	result, err := divide(10, 2)
	if err != nil {
		fmt.Printf("エラー: %v\n", err)
	} else {
		fmt.Printf("結果: %.2f\n", result)
	}

	// エラーの場合
	result, err = divide(10, 0)
	if err != nil {
		fmt.Printf("エラー: %v\n", err)
	} else {
		fmt.Printf("結果: %.2f\n", result)
	}

	// カスタムエラー
	user, err := validateUser("", 15, "invalid-email")
	if err != nil {
		fmt.Printf("バリデーションエラー: %v\n", err)
	} else {
		fmt.Printf("ユーザー: %+v\n", user)
	}

	// エラーのラッピング
	err = processFile("nonexistent.txt")
	if err != nil {
		fmt.Printf("ファイル処理エラー: %v\n", err)
	}
}

func divide(a, b float64) (float64, error) {
	if b == 0 {
		return 0, errors.New("ゼロで割ることはできません")
	}
	return a / b, nil
}

type User struct {
	Name  string
	Age   int
	Email string
}

type ValidationError struct {
	Field   string
	Message string
}

func (e ValidationError) Error() string {
	return fmt.Sprintf("フィールド '%s': %s", e.Field, e.Message)
}

func validateUser(name string, age int, email string) (User, error) {
	if name == "" {
		return User{}, ValidationError{Field: "name", Message: "名前が空です"}
	}
	if age < 18 {
		return User{}, ValidationError{Field: "age", Message: "18歳未満です"}
	}
	if !strings.Contains(email, "@") {
		return User{}, ValidationError{Field: "email", Message: "無効なメールアドレスです"}
	}
	
	return User{Name: name, Age: age, Email: email}, nil
}

func processFile(filename string) error {
	err := readFile(filename)
	if err != nil {
		return fmt.Errorf("ファイル処理に失敗: %w", err)
	}
	return nil
}

func readFile(filename string) error {
	return errors.New("ファイルが見つかりません")
}