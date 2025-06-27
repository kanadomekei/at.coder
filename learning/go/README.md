# Go 文法完全ガイド

Goは、Googleが開発したシンプルで効率的なシステムプログラミング言語です。この資料では、Goの文法を基礎から応用まで詳しく解説します。

## 📚 目次

1. [基本文法](#基本文法)
2. [変数とデータ型](#変数とデータ型)
3. [制御構文](#制御構文)
4. [関数](#関数)
5. [構造体とインターフェース](#構造体とインターフェース)
6. [エラーハンドリング](#エラーハンドリング)
7. [並行処理](#並行処理)
8. [パッケージとモジュール](#パッケージとモジュール)
9. [実行方法](#実行方法)

---

## 基本文法

### プログラム構造

```go
package main

import "fmt"

func main() {
    fmt.Println("Hello, Go!")
}
```

- **パッケージ**: 全てのGoファイルは`package`宣言で始まる
- **main関数**: `main`パッケージの`main`関数がエントリーポイント
- **import**: 必要なパッケージを読み込み
- **大文字**: 関数名や変数名が大文字で始まると外部パッケージから参照可能

### コメント

```go
// 単行コメント

/*
   複数行コメント
   ここも含まれる
*/

// パッケージドキュメント（パッケージ宣言直前）
// このパッケージは基本的な計算を行います。
package calculator

// 関数ドキュメント（関数宣言直前）
// Add は二つの整数を足し算します。
func Add(a, b int) int {
    return a + b
}
```

---

## 変数とデータ型

### 変数宣言

```go
// var による宣言
var name string
name = "Alice"

// 初期化付き宣言
var age int = 30
var height float64 = 165.5

// 型推論
var active = true // bool型として推論

// 短縮変数宣言（関数内のみ）
email := "alice@example.com"

// 複数変数の同時宣言
var x, y int = 10, 20
a, b := "hello", 42

// 変数グループ宣言
var (
    username    string  = "bob"
    userAge     int     = 25
    userBalance float64 = 1000.50
)
```

**重要な規則:**
- 関数外では`var`キーワードが必須
- `:=`は関数内でのみ使用可能
- 宣言された変数は必ず使用する必要がある

### データ型

#### 整数型

| 型 | サイズ | 範囲 |
|---|---|---|
| `int8` | 8bit | -128 〜 127 |
| `int16` | 16bit | -32,768 〜 32,767 |
| `int32` | 32bit | -2³¹ 〜 2³¹-1 |
| `int64` | 64bit | -2⁶³ 〜 2⁶³-1 |
| `int` | プラットフォーム依存 | 32または64bit |
| `uint8` | 8bit | 0 〜 255 |
| `uint16` | 16bit | 0 〜 65,535 |
| `uint32` | 32bit | 0 〜 2³²-1 |
| `uint64` | 64bit | 0 〜 2⁶⁴-1 |
| `uint` | プラットフォーム依存 | 32または64bit |

```go
var a int = 42
var b uint64 = 100
var c = 1_000_000 // アンダースコアで読みやすく

// 特別な整数型
var byteVal byte = 255    // uint8のエイリアス
var runeVal rune = 'A'    // int32のエイリアス（Unicode文字）
var ptr uintptr = 0x1000  // ポインタサイズの符号なし整数
```

#### 浮動小数点型

```go
var f32 float32 = 3.14    // 32bit単精度
var f64 float64 = 2.71828 // 64bit倍精度（デフォルト）

// 科学記法
var large = 1e6    // 1,000,000
var small = 2.5e-3 // 0.0025
```

#### 複素数型

```go
var c64 complex64 = 1 + 2i   // 32bit実部+32bit虚部
var c128 complex128 = 3 + 4i // 64bit実部+64bit虚部

// 複素数の操作
fmt.Printf("実部: %g, 虚部: %g\n", real(c128), imag(c128))
```

#### 論理型

```go
var isActive bool = true
var isComplete bool = false

// 論理演算
result := isActive && isComplete // AND
result = isActive || isComplete  // OR
result = !isActive               // NOT
```

#### 文字列型

```go
var greeting string = "Hello, Go!"

// 生文字列（エスケープシーケンス無効）
var multiline string = `これは
複数行の
文字列です`

// 文字列操作
len := len(greeting)        // 文字列の長さ
first := greeting[0]        // 最初のバイト
concat := greeting + " Hi!" // 連結

// Unicode文字の処理
japanese := "こんにちは"
for i, r := range japanese {
    fmt.Printf("位置 %d: %c\n", i, r)
}
```

### 定数

```go
const Pi = 3.14159
const MaxUsers = 1000
const AppName = "MyGoApp"

// 定数グループ
const (
    StatusOK       = 200
    StatusNotFound = 404
    StatusError    = 500
)

// iota を使った連続値
const (
    Monday = iota // 0
    Tuesday       // 1
    Wednesday     // 2
    Thursday      // 3
    Friday        // 4
    Saturday      // 5
    Sunday        // 6
)
```

### 型変換

```go
var i int = 42
var f float64 = float64(i)  // 明示的な型変換
var u uint = uint(f)

// 文字列と数値の変換
import "strconv"

// 数値から文字列
numStr := strconv.Itoa(123)
floatStr := strconv.FormatFloat(3.14159, 'f', 2, 64)

// 文字列から数値
num, err := strconv.Atoi("456")
floatVal, err := strconv.ParseFloat("2.718", 64)
```

### ゼロ値

```go
var zeroInt int           // 0
var zeroFloat float64     // 0.0
var zeroBool bool         // false
var zeroString string     // ""
var zeroSlice []int       // nil
var zeroMap map[string]int // nil
```

---

## 制御構文

### if文

```go
// 基本的なif文
if x > 0 {
    fmt.Println("正の数")
} else if x < 0 {
    fmt.Println("負の数")
} else {
    fmt.Println("ゼロ")
}

// 初期化付きif文
if num := getValue(); num > 100 {
    fmt.Println("大きな値:", num)
}

// エラーチェックの慣用パターン
if err := someFunction(); err != nil {
    log.Fatal(err)
}
```

### for文（ループ）

```go
// 基本的なfor文
for i := 0; i < 10; i++ {
    fmt.Println(i)
}

// while文として使用
i := 0
for i < 10 {
    fmt.Println(i)
    i++
}

// 無限ループ
for {
    // 何かの処理
    if condition {
        break
    }
}

// range を使った反復
slice := []int{1, 2, 3, 4, 5}

// インデックスと値
for i, v := range slice {
    fmt.Printf("インデックス %d: 値 %d\n", i, v)
}

// 値のみ
for _, v := range slice {
    fmt.Println(v)
}

// インデックスのみ
for i := range slice {
    fmt.Println(i)
}

// マップの反復
m := map[string]int{"a": 1, "b": 2}
for key, value := range m {
    fmt.Printf("%s: %d\n", key, value)
}

// 文字列の反復（rune）
s := "Hello"
for i, r := range s {
    fmt.Printf("位置 %d: %c\n", i, r)
}
```

### switch文

```go
// 基本的なswitch文
switch day {
case "Monday":
    fmt.Println("月曜日")
case "Tuesday":
    fmt.Println("火曜日")
default:
    fmt.Println("その他")
}

// 複数の値をマッチ
switch day {
case "Saturday", "Sunday":
    fmt.Println("週末")
default:
    fmt.Println("平日")
}

// 条件式付きswitch
switch {
case score >= 90:
    fmt.Println("A")
case score >= 80:
    fmt.Println("B")
case score >= 70:
    fmt.Println("C")
default:
    fmt.Println("F")
}

// 型switch
switch v := x.(type) {
case int:
    fmt.Printf("整数: %d\n", v)
case string:
    fmt.Printf("文字列: %s\n", v)
case []int:
    fmt.Printf("整数スライス: %v\n", v)
default:
    fmt.Printf("不明な型: %T\n", v)
}
```

### defer文

```go
func example() {
    defer fmt.Println("この文は関数終了時に実行される")
    
    fmt.Println("通常の処理")
    
    // 複数のdefer（LIFO順で実行）
    defer fmt.Println("3番目に実行")
    defer fmt.Println("2番目に実行")
    defer fmt.Println("1番目に実行")
}

// リソース管理の典型的な使用例
func readFile(filename string) {
    file, err := os.Open(filename)
    if err != nil {
        return
    }
    defer file.Close() // 関数終了時に確実にクローズ
    
    // ファイル操作
}
```

---

## 関数

### 基本的な関数

```go
// 基本的な関数
func greet(name string) {
    fmt.Printf("Hello, %s!\n", name)
}

// 戻り値あり
func add(x, y int) int {
    return x + y
}

// 複数の引数が同じ型
func multiply(x, y, z int) int {
    return x * y * z
}

// 複数の戻り値
func divide(x, y float64) (float64, error) {
    if y == 0 {
        return 0, errors.New("division by zero")
    }
    return x / y, nil
}

// 名前付き戻り値
func split(sum int) (x, y int) {
    x = sum * 4 / 9
    y = sum - x
    return // naked return
}
```

### 可変長引数

```go
// 可変長引数
func sum(numbers ...int) int {
    total := 0
    for _, num := range numbers {
        total += num
    }
    return total
}

// 使用例
result := sum(1, 2, 3, 4, 5)
slice := []int{1, 2, 3}
result = sum(slice...) // スライスを展開
```

### 関数型と高階関数

```go
// 関数型
type Operation func(int, int) int

// 関数を引数として受け取る
func calculate(x, y int, op Operation) int {
    return op(x, y)
}

// 関数を戻り値として返す
func getMultiplier(factor int) func(int) int {
    return func(x int) int {
        return x * factor
    }
}

// 使用例
add := func(a, b int) int { return a + b }
result := calculate(5, 3, add)

double := getMultiplier(2)
result = double(10) // 20
```

### クロージャ

```go
// クロージャの例
func counter() func() int {
    count := 0
    return func() int {
        count++
        return count
    }
}

// 使用例
c1 := counter()
c2 := counter()

fmt.Println(c1()) // 1
fmt.Println(c1()) // 2
fmt.Println(c2()) // 1（別のカウンター）
```

### メソッド

```go
type Person struct {
    Name string
    Age  int
}

// 値レシーバー
func (p Person) String() string {
    return fmt.Sprintf("%s (%d歳)", p.Name, p.Age)
}

// ポインタレシーバー（値を変更する場合）
func (p *Person) Birthday() {
    p.Age++
}

// 使用例
p := Person{Name: "Alice", Age: 30}
fmt.Println(p.String()) // Alice (30歳)
p.Birthday()
fmt.Println(p.Age) // 31
```

---

## 構造体とインターフェース

### 構造体

```go
// 基本的な構造体
type User struct {
    ID       int
    Name     string
    Email    string
    IsActive bool
}

// 構造体の初期化
user1 := User{
    ID:       1,
    Name:     "Alice",
    Email:    "alice@example.com",
    IsActive: true,
}

// フィールド名省略（順序重要）
user2 := User{2, "Bob", "bob@example.com", false}

// 一部フィールドのみ指定
user3 := User{Name: "Charlie"}

// ポインタから構造体を作成
user4 := &User{Name: "David"}
```

### 埋め込み（Embedding）

```go
type Address struct {
    Street   string
    City     string
    Zip      string
}

type Person struct {
    Name    string
    Age     int
    Address // 埋め込み
}

// 使用例
p := Person{
    Name: "Alice",
    Age:  30,
    Address: Address{
        Street: "123 Main St",
        City:   "Tokyo",
        Zip:    "100-0001",
    },
}

// 埋め込みフィールドへの直接アクセス
fmt.Println(p.Street) // p.Address.Street と同じ
```

### インターフェース

```go
// インターフェースの定義
type Writer interface {
    Write([]byte) (int, error)
}

type Reader interface {
    Read([]byte) (int, error)
}

// インターフェースの組み合わせ
type ReadWriter interface {
    Reader
    Writer
}

// 空インターフェース（任意の型を受け取る）
func print(v interface{}) {
    fmt.Println(v)
}
```

### インターフェースの実装

```go
type File struct {
    name string
}

// File は Writer インターフェースを実装
func (f *File) Write(data []byte) (int, error) {
    fmt.Printf("Writing to %s: %s\n", f.name, string(data))
    return len(data), nil
}

// 暗黙的な実装（implementsキーワード不要）
func writeData(w Writer, data []byte) {
    w.Write(data)
}

// 使用例
f := &File{name: "test.txt"}
writeData(f, []byte("Hello, Go!"))
```

### 型アサーション

```go
var x interface{} = "hello"

// 型アサーション
s := x.(string)
fmt.Println(s)

// 安全な型アサーション
if s, ok := x.(string); ok {
    fmt.Println("文字列:", s)
}

// 型switch
switch v := x.(type) {
case string:
    fmt.Printf("文字列: %s\n", v)
case int:
    fmt.Printf("整数: %d\n", v)
default:
    fmt.Printf("不明な型: %T\n", v)
}
```

---

## エラーハンドリング

### error インターフェース

```go
type error interface {
    Error() string
}

// 基本的なエラーハンドリング
func divide(a, b float64) (float64, error) {
    if b == 0 {
        return 0, errors.New("division by zero")
    }
    return a / b, nil
}

result, err := divide(10, 0)
if err != nil {
    fmt.Printf("エラー: %v\n", err)
    return
}
fmt.Printf("結果: %.2f\n", result)
```

### カスタムエラー型

```go
type ValidationError struct {
    Field   string
    Message string
}

func (e ValidationError) Error() string {
    return fmt.Sprintf("フィールド '%s': %s", e.Field, e.Message)
}

func validateUser(name string, age int) error {
    if name == "" {
        return ValidationError{Field: "name", Message: "名前が空です"}
    }
    if age < 18 {
        return ValidationError{Field: "age", Message: "18歳未満です"}
    }
    return nil
}
```

### エラーのラッピング（Go 1.13+）

```go
import "fmt"

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

// エラーの確認
err := processFile("test.txt")
if err != nil {
    fmt.Println(err)
    
    // 元のエラーを取得
    if originalErr := errors.Unwrap(err); originalErr != nil {
        fmt.Println("元のエラー:", originalErr)
    }
}
```

### パニックとリカバー

```go
// panic（プログラムを停止）
func divide(a, b int) int {
    if b == 0 {
        panic("division by zero")
    }
    return a / b
}

// recover（パニックから回復）
func safeCall() {
    defer func() {
        if r := recover(); r != nil {
            fmt.Printf("パニックから回復: %v\n", r)
        }
    }()
    
    divide(10, 0) // パニックが発生
    fmt.Println("この行は実行されない")
}
```

---

## 並行処理

### ゴルーチン

```go
// 基本的なゴルーチン
func sayHello() {
    fmt.Println("Hello from goroutine!")
}

func main() {
    go sayHello() // 新しいゴルーチンで実行
    
    // 匿名関数のゴルーチン
    go func() {
        fmt.Println("Anonymous goroutine!")
    }()
    
    time.Sleep(time.Second) // メインゴルーチンを待機
}
```

### チャネル

```go
// チャネルの作成
ch := make(chan string)

// 値の送信（ブロッキング）
go func() {
    ch <- "Hello"
}()

// 値の受信（ブロッキング）
message := <-ch
fmt.Println(message)

// バッファ付きチャネル
bufferedCh := make(chan int, 3)
bufferedCh <- 1
bufferedCh <- 2
bufferedCh <- 3
// バッファが満杯になるまでブロックしない
```

### チャネルの操作

```go
// チャネルのクローズ
ch := make(chan int)
go func() {
    for i := 0; i < 5; i++ {
        ch <- i
    }
    close(ch) // チャネルをクローズ
}()

// range でチャネルから値を受信
for value := range ch {
    fmt.Println(value)
}

// チャネルがクローズされているかチェック
value, ok := <-ch
if !ok {
    fmt.Println("チャネルがクローズされています")
}
```

### select文

```go
// 複数のチャネル操作を待機
ch1 := make(chan string)
ch2 := make(chan string)

go func() {
    time.Sleep(time.Second)
    ch1 <- "from ch1"
}()

go func() {
    time.Sleep(2 * time.Second)
    ch2 <- "from ch2"
}()

select {
case msg1 := <-ch1:
    fmt.Println(msg1)
case msg2 := <-ch2:
    fmt.Println(msg2)
case <-time.After(3 * time.Second):
    fmt.Println("タイムアウト")
default:
    fmt.Println("デフォルトケース")
}
```

### sync パッケージ

```go
import "sync"

// Mutex（排他制御）
var (
    counter int
    mutex   sync.Mutex
)

func increment() {
    mutex.Lock()
    defer mutex.Unlock()
    counter++
}

// WaitGroup（ゴルーチンの完了待機）
var wg sync.WaitGroup

func worker(id int) {
    defer wg.Done()
    fmt.Printf("Worker %d is working\n", id)
}

func main() {
    for i := 1; i <= 3; i++ {
        wg.Add(1)
        go worker(i)
    }
    wg.Wait() // 全てのワーカーの完了を待機
}

// Once（一度だけ実行）
var once sync.Once

func initialize() {
    fmt.Println("初期化処理")
}

func doSomething() {
    once.Do(initialize) // 複数回呼んでも一度だけ実行
}
```

---

## パッケージとモジュール

### パッケージ

```go
// パッケージ宣言（ファイルの最初）
package calculator

// エクスポートされる関数（大文字開始）
func Add(a, b int) int {
    return a + b
}

// パッケージ内でのみ利用可能（小文字開始）
func subtract(a, b int) int {
    return a - b
}

// init関数（パッケージ読み込み時に自動実行）
func init() {
    fmt.Println("calculator package initialized")
}
```

### import文

```go
// 基本的なimport
import "fmt"
import "math"

// グループimport
import (
    "fmt"
    "math"
    "net/http"
)

// エイリアス
import (
    f "fmt"           // fmt を f として使用
    m "math"          // math を m として使用
    . "strings"       // strings の関数を直接使用
    _ "image/png"     // 副作用のためだけにimport
)

// 使用例
f.Println("Hello")     // fmt.Println の代わり
result := m.Sqrt(16)   // math.Sqrt の代わり
str := ToUpper("hello") // strings.ToUpper の代わり
```

### モジュール（Go 1.11+）

```bash
# 新しいモジュールの作成
go mod init example.com/myproject

# 依存関係の追加
go get github.com/gorilla/mux

# 依存関係の整理
go mod tidy

# ベンダリング
go mod vendor
```

```go
// go.mod ファイル
module example.com/myproject

go 1.21

require (
    github.com/gorilla/mux v1.8.0
    golang.org/x/crypto v0.0.0-20210921155107-089bfa567519
)
```

---

## 実行方法

### ファイル実行

```bash
# 各サンプルファイルの実行
go run 01_variables_types.go
go run 02_control_flow.go
go run 03_functions.go
go run 04_structs_interfaces.go
go run 05_error_handling.go
go run 06_goroutines_channels.go
go run 07_packages_modules.go
go run 08_practical_examples.go
```

### プロジェクト管理

```bash
# 新しいプロジェクトの作成
mkdir myproject
cd myproject
go mod init myproject

# main.go を作成してコードを書く

# 実行
go run .
# または
go run main.go

# ビルド
go build
# 実行可能ファイルが生成される

# クロスコンパイル
GOOS=linux GOARCH=amd64 go build
GOOS=windows GOARCH=amd64 go build

# テスト実行
go test

# フォーマット
go fmt

# 静的解析
go vet

# 依存関係の確認
go list -m all

# プロファイリング
go run -cpuprofile=cpu.prof main.go
```

### Go tools

```bash
# go doc（ドキュメント表示）
go doc fmt.Println

# go fix（コードの自動修正）
go fix

# go generate（コード生成）
go generate

# go install（バイナリのインストール）
go install

# go clean（ビルド成果物の削除）
go clean
```

---

## 🎯 学習のポイント

### 重要な概念の理解順序

1. **基本文法**: 変数、関数、制御構文
2. **構造体とインターフェース**: Goの型システム
3. **エラーハンドリング**: 明示的なエラー処理
4. **並行処理**: ゴルーチンとチャネル
5. **パッケージ**: コードの組織化

### Goの哲学

1. **シンプリシティ**: 複雑さを避け、明確で読みやすいコード
2. **コンポジション**: 継承ではなく合成を好む
3. **明示性**: 暗黙的な動作を避ける
4. **並行処理**: "Don't communicate by sharing memory; share memory by communicating"
5. **実用性**: 実用的な問題解決を重視

### よくある学習の躓きポイント

1. **ポインタとレシーバー**: いつ値レシーバー、いつポインタレシーバーを使うか
2. **インターフェース**: 暗黙的な実装の概念
3. **チャネル**: デッドロックの回避
4. **エラーハンドリング**: 例外ではなく値としてのエラー

---

## 📚 次のステップ

1. [A Tour of Go](https://tour.golang.org/)で実践的な演習
2. [Effective Go](https://golang.org/doc/effective_go.html)でGoらしい書き方を学習
3. 標準ライブラリのソースコードを読む
4. 小さなWebサービスやCLIツールを作成
5. Go Playgroundでコードを試す

Goは学習コストが低く、実用的な言語です。"less is more"の哲学を理解して、シンプルで効率的なコードを書きましょう！