package main

import (
	"fmt"
	"math"
)

// 構造体の定義
type Person struct {
	Name    string
	Age     int
	Email   string
	Address Address // 埋め込み構造体
}

type Address struct {
	Street   string
	City     string
	ZipCode  string
	Country  string
}

// インターフェースの定義
type Shape interface {
	Area() float64
	Perimeter() float64
}

type Drawable interface {
	Draw()
}

// 複数のインターフェースを埋め込み
type ShapeDrawer interface {
	Shape
	Drawable
}

// 構造体の実装
type Circle struct {
	Radius float64
	Center Point2D
}

type Rectangle2 struct {
	Width  float64
	Height float64
}

type Point2D struct {
	X, Y float64
}

// Circleのメソッド実装
func (c Circle) Area() float64 {
	return math.Pi * c.Radius * c.Radius
}

func (c Circle) Perimeter() float64 {
	return 2 * math.Pi * c.Radius
}

func (c Circle) Draw() {
	fmt.Printf("円を描画: 中心(%.1f, %.1f), 半径%.1f\n", c.Center.X, c.Center.Y, c.Radius)
}

// Rectangle2のメソッド実装
func (r Rectangle2) Area() float64 {
	return r.Width * r.Height
}

func (r Rectangle2) Perimeter() float64 {
	return 2 * (r.Width + r.Height)
}

func (r Rectangle2) Draw() {
	fmt.Printf("長方形を描画: 幅%.1f, 高さ%.1f\n", r.Width, r.Height)
}

func main() {
	fmt.Println("=== Goの構造体とインターフェース ===")

	// 構造体の初期化
	fmt.Println("\n=== 構造体の初期化 ===")

	// フィールド名を指定した初期化
	person1 := Person{
		Name: "Alice",
		Age:  30,
		Email: "alice@example.com",
		Address: Address{
			Street:  "123 Main St",
			City:    "Tokyo",
			ZipCode: "100-0001",
			Country: "Japan",
		},
	}

	// 順序による初期化（推奨されない）
	person2 := Person{
		"Bob",
		25,
		"bob@example.com",
		Address{"456 Second St", "Osaka", "540-0001", "Japan"},
	}

	// 一部のフィールドのみ初期化
	person3 := Person{
		Name: "Charlie",
		Age:  35,
	}

	fmt.Printf("Person1: %+v\n", person1)
	fmt.Printf("Person2: %+v\n", person2)
	fmt.Printf("Person3: %+v\n", person3)

	// フィールドへのアクセス
	fmt.Printf("Person1の名前: %s\n", person1.Name)
	fmt.Printf("Person1の住所: %s, %s\n", person1.Address.City, person1.Address.Country)

	// 構造体のポインタ
	fmt.Println("\n=== 構造体のポインタ ===")

	person4 := &Person{
		Name:  "David",
		Age:   28,
		Email: "david@example.com",
	}

	// ポインタ経由でのアクセス（自動的にデリファレンス）
	fmt.Printf("Person4の名前: %s\n", person4.Name)
	fmt.Printf("Person4の年齢: %d\n", (*person4).Age) // 明示的なデリファレンス

	// ポインタの比較
	person5 := person4
	fmt.Printf("person4 == person5: %t\n", person4 == person5)

	// インターフェースの使用
	fmt.Println("\n=== インターフェースの使用 ===")

	circle := Circle{
		Radius: 5.0,
		Center: Point2D{X: 0, Y: 0},
	}

	rectangle := Rectangle2{
		Width:  10.0,
		Height: 6.0,
	}

	// インターフェース型の変数
	var shape Shape

	shape = circle
	fmt.Printf("円の面積: %.2f\n", shape.Area())
	fmt.Printf("円の周囲: %.2f\n", shape.Perimeter())

	shape = rectangle
	fmt.Printf("長方形の面積: %.2f\n", shape.Area())
	fmt.Printf("長方形の周囲: %.2f\n", shape.Perimeter())

	// インターフェーススライス
	shapes := []Shape{circle, rectangle}

	fmt.Println("すべての図形:")
	for i, s := range shapes {
		fmt.Printf("図形 %d: 面積=%.2f, 周囲=%.2f\n", i+1, s.Area(), s.Perimeter())
	}

	// 型アサーション
	fmt.Println("\n=== 型アサーション ===")

	var drawable interface{} = circle

	// 成功する型アサーション
	if c, ok := drawable.(Circle); ok {
		fmt.Printf("Circle型として認識: 半径%.1f\n", c.Radius)
	}

	// 失敗する型アサーション
	if r, ok := drawable.(Rectangle2); ok {
		fmt.Printf("Rectangle2型として認識: 幅%.1f\n", r.Width)
	} else {
		fmt.Println("Rectangle2型ではありません")
	}

	// 型switch
	fmt.Println("\n=== 型switch ===")

	values := []interface{}{circle, rectangle, 42, "hello", person1}

	for i, value := range values {
		switch v := value.(type) {
		case Circle:
			fmt.Printf("値 %d: 円（半径%.1f）\n", i, v.Radius)
		case Rectangle2:
			fmt.Printf("値 %d: 長方形（%.1f x %.1f）\n", i, v.Width, v.Height)
		case Person:
			fmt.Printf("値 %d: 人（%s, %d歳）\n", i, v.Name, v.Age)
		case int:
			fmt.Printf("値 %d: 整数（%d）\n", i, v)
		case string:
			fmt.Printf("値 %d: 文字列（%s）\n", i, v)
		default:
			fmt.Printf("値 %d: 不明な型（%T）\n", i, v)
		}
	}

	// 空のインターフェース
	fmt.Println("\n=== 空のインターフェース ===")

	var anything interface{}

	anything = 42
	fmt.Printf("anything = %v (型: %T)\n", anything, anything)

	anything = "hello"
	fmt.Printf("anything = %v (型: %T)\n", anything, anything)

	anything = circle
	fmt.Printf("anything = %v (型: %T)\n", anything, anything)

	// 複数のインターフェースの実装
	fmt.Println("\n=== 複数のインターフェースの実装 ===")

	var shapeDrawer ShapeDrawer = circle

	fmt.Printf("ShapeDrawer の面積: %.2f\n", shapeDrawer.Area())
	shapeDrawer.Draw()

	// メソッドセット
	fmt.Println("\n=== メソッドセット ===")

	var drawables []Drawable = []Drawable{circle, rectangle}

	fmt.Println("描画可能なオブジェクト:")
	for _, d := range drawables {
		d.Draw()
	}

	// 構造体の埋め込み
	fmt.Println("\n=== 構造体の埋め込み ===")

	type Employee struct {
		Person   // 埋め込み
		ID       int
		Position string
		Salary   float64
	}

	emp := Employee{
		Person: Person{
			Name:  "Eve",
			Age:   32,
			Email: "eve@company.com",
		},
		ID:       1001,
		Position: "Software Engineer",
		Salary:   75000.0,
	}

	// 埋め込みフィールドへの直接アクセス
	fmt.Printf("従業員名: %s\n", emp.Name) // emp.Person.Name と同じ
	fmt.Printf("従業員ID: %d\n", emp.ID)
	fmt.Printf("職位: %s\n", emp.Position)

	// ポインタを使った構造体の更新
	fmt.Println("\n=== 構造体の更新 ===")

	updatePersonAge(&person1, 31)
	fmt.Printf("更新後のPerson1: %+v\n", person1)

	// 値渡しの場合（元の構造体は変更されない）
	updatePersonAgeByValue(person1, 32)
	fmt.Printf("値渡し後のPerson1: %+v\n", person1) // 変更されない

	// 構造体の比較
	fmt.Println("\n=== 構造体の比較 ===")

	addr1 := Address{Street: "123 Main St", City: "Tokyo"}
	addr2 := Address{Street: "123 Main St", City: "Tokyo"}
	addr3 := Address{Street: "456 Second St", City: "Osaka"}

	fmt.Printf("addr1 == addr2: %t\n", addr1 == addr2)
	fmt.Printf("addr1 == addr3: %t\n", addr1 == addr3)

	// 匿名構造体
	fmt.Println("\n=== 匿名構造体 ===")

	config := struct {
		Debug   bool
		Timeout int
		Host    string
	}{
		Debug:   true,
		Timeout: 30,
		Host:    "localhost",
	}

	fmt.Printf("設定: %+v\n", config)
}

// ヘルパー関数
func updatePersonAge(p *Person, newAge int) {
	p.Age = newAge
}

func updatePersonAgeByValue(p Person, newAge int) {
	p.Age = newAge // ローカルコピーのみ変更
}

// カスタムエラー型の実装
type ValidationError struct {
	Field   string
	Message string
}

func (e ValidationError) Error() string {
	return fmt.Sprintf("validation error in field '%s': %s", e.Field, e.Message)
}

// Stringerインターフェースの実装
func (p Person) String() string {
	return fmt.Sprintf("%s (%d歳)", p.Name, p.Age)
}

func (a Address) String() string {
	return fmt.Sprintf("%s, %s %s, %s", a.Street, a.City, a.ZipCode, a.Country)
}