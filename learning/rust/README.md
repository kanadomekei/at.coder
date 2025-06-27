# Rust 文法完全ガイド

Rustは、メモリ安全性とパフォーマンスを両立させるシステムプログラミング言語です。この資料では、Rustの文法を基礎から応用まで詳しく解説します。

## 📚 目次

1. [基本文法](#基本文法)
2. [変数とデータ型](#変数とデータ型)
3. [制御構文](#制御構文)
4. [関数](#関数)
5. [構造体とenum](#構造体とenum)
6. [所有権システム](#所有権システム)
7. [エラーハンドリング](#エラーハンドリング)
8. [トレイトとジェネリクス](#トレイトとジェネリクス)
9. [実行方法](#実行方法)

---

## 基本文法

### プログラム構造

```rust
fn main() {
    println!("Hello, Rust!");
}
```

- **エントリーポイント**: `main`関数がプログラムの開始点
- **マクロ**: `println!`は標準出力マクロ（`!`がマクロの印）
- **セミコロン**: 文の終わりには`;`が必要

### コメント

```rust
// 単行コメント

/* 
   複数行コメント
*/

/// ドキュメントコメント（関数やモジュールの説明）
/// `cargo doc`でHTMLドキュメントを生成
fn documented_function() {}
```

---

## 変数とデータ型

### 変数宣言

```rust
// 不変変数（デフォルト）
let x = 5;

// 可変変数
let mut y = 10;
y = 15; // OK

// 型注釈
let z: i32 = 20;

// 定数（コンパイル時定数）
const MAX_POINTS: u32 = 100_000;
```

**重要な概念:**
- **不変性**: Rustのデフォルトは不変（immutable）
- **シャドーイング**: 同じ名前で新しい変数を宣言可能

```rust
let x = 5;
let x = x + 1; // 新しいx（シャドーイング）
let x = "hello"; // 型も変更可能
```

### データ型

#### 整数型

| 型 | 範囲 | 用途 |
|---|---|---|
| `i8` | -128 〜 127 | 小さな数値 |
| `i16` | -32,768 〜 32,767 | 中程度の数値 |
| `i32` | -2³¹ 〜 2³¹-1 | デフォルト整数型 |
| `i64` | -2⁶³ 〜 2⁶³-1 | 大きな数値 |
| `i128` | -2¹²⁷ 〜 2¹²⁷-1 | 非常に大きな数値 |
| `isize` | プラットフォーム依存 | インデックスサイズ |

```rust
let a: i32 = -42;
let b: u64 = 100; // 符号なし
let c = 1_000_000; // アンダースコアで読みやすく
let d = 0xff; // 16進数
let e = 0o77; // 8進数
let f = 0b1111; // 2進数
```

#### 浮動小数点型

```rust
let x: f32 = 3.14; // 32bit単精度
let y: f64 = 2.71828; // 64bit倍精度（デフォルト）
```

#### 論理型

```rust
let t: bool = true;
let f = false; // 型推論
```

#### 文字・文字列型

```rust
let c: char = 'z'; // Unicode文字（4byte）
let emoji = '😀'; // 絵文字も可能

let s: &str = "hello"; // 文字列スライス
let s2: String = String::from("world"); // 所有権を持つ文字列
```

#### タプル型

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup; // 分解
let five_hundred = tup.0; // インデックスアクセス
```

#### 配列型

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5]; // 固定長
let b = [3; 5]; // [3, 3, 3, 3, 3]
let first = a[0]; // インデックスアクセス
```

---

## 制御構文

### if文

```rust
let number = 6;

if number % 4 == 0 {
    println!("4で割り切れる");
} else if number % 3 == 0 {
    println!("3で割り切れる");
} else {
    println!("4でも3でも割り切れない");
}

// if式（値を返す）
let condition = true;
let number = if condition { 5 } else { 6 };
```

### ループ

#### loop（無限ループ）

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    
    if counter == 10 {
        break counter * 2; // 値を返してbreak
    }
};
```

#### while

```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

#### for

```rust
let a = [10, 20, 30, 40, 50];

// 配列の要素を反復
for element in a {
    println!("値: {}", element);
}

// 範囲を反復
for number in 1..4 { // 1, 2, 3
    println!("{}", number);
}

// インデックス付きfor
for (i, value) in a.iter().enumerate() {
    println!("インデックス{}: {}", i, value);
}
```

### match文（パターンマッチング）

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"), // その他すべて
}

// 値を返すmatch
let x = 1;
let message = match x {
    1 => "one",
    2 => "two",
    _ => "other",
};

// パターンガード
let x = Some(5);
match x {
    Some(n) if n < 5 => println!("5未満: {}", n),
    Some(n) => println!("5以上: {}", n),
    None => println!("値なし"),
}
```

---

## 関数

### 基本的な関数

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y // 最後の式が返り値（returnは省略可能）
}

fn explicit_return(x: i32) -> i32 {
    return x * 2; // 明示的なreturn
}
```

### 高度な関数

```rust
// 複数の返り値（タプル）
fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

// 関数を変数に代入
let f = add;
let result = f(1, 2);

// 関数を引数として受け取る
fn apply_twice<F>(f: F, x: i32) -> i32 
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

let double = |x| x * 2; // クロージャ
let result = apply_twice(double, 3); // 12
```

### クロージャ

```rust
// 基本的なクロージャ
let add_one = |x| x + 1;

// 型注釈付き
let add_one = |x: i32| -> i32 { x + 1 };

// 環境をキャプチャ
let x = 10;
let capture_x = |y| x + y;

// moveクロージャ（所有権を移動）
let s = String::from("hello");
let closure = move |name| format!("{}, {}", s, name);
```

---

## 構造体とenum

### 構造体

```rust
// 基本的な構造体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// インスタンス作成
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// フィールド初期化省略記法
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 構造体更新記法
let user2 = User {
    email: String::from("another@example.com"),
    ..user1 // user1の残りのフィールドをコピー
};
```

### タプル構造体

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

### メソッド

```rust
impl User {
    // メソッド
    fn sign_in(&mut self) {
        self.sign_in_count += 1;
    }
    
    // 関連関数（静的メソッド）
    fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}

// 使用例
let mut user = User::new(
    String::from("test@example.com"),
    String::from("testuser")
);
user.sign_in();
```

### enum

```rust
// 基本的なenum
enum IpAddrKind {
    V4,
    V6,
}

// データを持つenum
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// enumとmatch
match home {
    IpAddr::V4(a, b, c, d) => {
        println!("IPv4: {}.{}.{}.{}", a, b, c, d);
    }
    IpAddr::V6(addr) => {
        println!("IPv6: {}", addr);
    }
}
```

### Option型

```rust
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;

// if letでの処理
if let Some(x) = some_number {
    println!("値: {}", x);
}

// matchでの処理
match some_number {
    Some(x) => println!("値: {}", x),
    None => println!("値なし"),
}
```

---

## 所有権システム

Rustの最も重要な概念です。

### 所有権の規則

1. Rustの各値は、**所有者**（owner）と呼ばれる変数を持つ
2. いかなる時も所有者は一つ
3. 所有者がスコープを抜けると、値は破棄される

```rust
{
    let s = String::from("hello"); // sが所有者
} // sがスコープを抜ける。sは破棄される
```

### ムーブ（所有権の移動）

```rust
let s1 = String::from("hello");
let s2 = s1; // s1からs2に所有権が移動

// println!("{}", s1); // エラー！s1はもう使えない
println!("{}", s2); // OK
```

### クローン

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // 明示的にコピー

println!("s1 = {}, s2 = {}", s1, s2); // 両方とも使える
```

### 参照と借用

```rust
fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1); // 参照を渡す
    
    println!("{} の長さは {} です", s1, len); // s1はまだ使える
}

fn calculate_length(s: &String) -> usize { // 参照を受け取る
    s.len()
} // sはスコープを抜けるが、参照なので何も破棄されない
```

### 可変参照

```rust
fn main() {
    let mut s = String::from("hello");
    
    change(&mut s); // 可変参照を渡す
    
    println!("{}", s); // "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### 参照の規則

```rust
let mut s = String::from("hello");

// 複数の不変参照は同時に持てる
let r1 = &s;
let r2 = &s;
println!("{} and {}", r1, r2);

// 可変参照は一つだけ
let r3 = &mut s;
println!("{}", r3);

// 不変参照と可変参照は同時に持てない
// let r4 = &s; // エラー！
```

### ライフタイム

```rust
// ライフタイム注釈が必要な例
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 構造体でのライフタイム
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

---

## エラーハンドリング

### Result型

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("ゼロで割ることはできません"))
    } else {
        Ok(x / y)
    }
}

// 使用例
match divide(4.0, 2.0) {
    Ok(result) => println!("結果: {}", result),
    Err(error) => println!("エラー: {}", error),
}
```

### ?演算子

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?; // エラーなら早期return
    let mut s = String::new();
    f.read_to_string(&mut s)?; // エラーなら早期return
    Ok(s)
}
```

### panic!マクロ

```rust
fn main() {
    panic!("クラッシュして燃える"); // プログラム終了
}

// 境界外アクセスでもpanicが発生
let v = vec![1, 2, 3];
v[99]; // panic!
```

### カスタムエラー型

```rust
use std::fmt;

#[derive(Debug)]
struct CalculationError {
    details: String
}

impl fmt::Display for CalculationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for CalculationError {}
```

---

## トレイトとジェネリクス

### トレイト

```rust
// トレイト定義
trait Summary {
    fn summarize(&self) -> String;
    
    // デフォルト実装
    fn summarize_default(&self) -> String {
        String::from("(続きを読む...)")
    }
}

// トレイト実装
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

### ジェネリクス

```rust
// 関数でのジェネリクス
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// 構造体でのジェネリクス
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

// トレイト境界
fn notify<T: Summary>(item: &T) {
    println!("速報: {}", item.summarize());
}

// where句を使った複雑なトレイト境界
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // 実装
    0
}
```

---

## 実行方法

### ファイル実行

```bash
# 各サンプルファイルの実行
rustc 01_variables_types.rs && ./01_variables_types
rustc 02_control_flow.rs && ./02_control_flow
rustc 03_functions.rs && ./03_functions
rustc 04_structs_enums.rs && ./04_structs_enums
rustc 05_ownership_lifetimes.rs && ./05_ownership_lifetimes
rustc 06_error_handling.rs && ./06_error_handling
rustc 07_traits_generics.rs && ./07_traits_generics
rustc 08_collections.rs && ./08_collections
rustc 09_practical_examples.rs && ./09_practical_examples
```

### Cargoプロジェクト

```bash
# 新しいプロジェクト作成
cargo new my_project
cd my_project

# 実行
cargo run

# リリースビルド
cargo build --release

# テスト実行
cargo test

# ドキュメント生成
cargo doc --open

# 依存関係チェック
cargo check
```

### Cargo.toml例

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

---

## 🎯 学習のポイント

### 重要な概念の理解順序

1. **基本文法**: 変数、関数、制御構文
2. **所有権**: Rustの核心概念
3. **エラーハンドリング**: Result型とOption型
4. **構造体とenum**: データ構造
5. **トレイト**: 抽象化とポリモーフィズム
6. **ジェネリクス**: 型安全なコード再利用

### よくある学習の躓きポイント

1. **所有権システム**: 最初は理解が困難。例をたくさん読んで慣れる
2. **ライフタイム**: 無理に理解しようとせず、コンパイラエラーから学ぶ
3. **match文**: 網羅性チェックを理解する
4. **トレイト境界**: 段階的に複雑さを増やして学習

---

## 📚 次のステップ

1. [The Rust Programming Language](https://doc.rust-lang.org/book/)を読破
2. [Rust by Example](https://doc.rust-lang.org/rust-by-example/)で実践
3. 小さなCLIツールを作成
4. WebAssemblyやAsync/Awaitに挑戦

Rustは学習コストは高いですが、一度理解すれば非常に強力な言語です。焦らず段階的に学習していきましょう！