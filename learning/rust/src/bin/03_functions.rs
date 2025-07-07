fn main() {
    // ==========================================
    // 関数
    // ==========================================
    
    println!("=== Rustの関数 ===");
    
    // ------------------------------------------
    // 基本的な関数呼び出し
    // ------------------------------------------
    
    println!("\n=== 基本的な関数 ===");
    
    // 引数なし、返り値なしの関数
    greet();
    
    // 引数あり、返り値なしの関数
    greet_person("Alice");
    
    // 引数あり、返り値ありの関数
    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);
    
    // 複数の返り値（タプル）
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {} 余り {}", quotient, remainder);
    
    // ------------------------------------------
    // 式と文
    // ------------------------------------------
    
    println!("\n=== 式と文 ===");
    
    // 式は値を返す（セミコロンなし）
    let x = {
        let a = 3;
        let b = 4;
        a + b  // 式：値を返す
    };
    println!("ブロック式の結果: {}", x);
    
    // 早期返却
    let result = early_return_example(5);
    println!("早期返却の結果: {}", result);
    
    // ------------------------------------------
    // 引数のパターン
    // ------------------------------------------
    
    println!("\n=== 引数のパターン ===");
    
    // タプルの分解
    let point = (3, 4);
    print_coordinates(point);
    
    // 構造体の分解
    let person = Person {
        name: String::from("Bob"),
        age: 25,
    };
    print_person_info(person);
    
    // 参照の受け取り
    let name = String::from("Charlie");
    print_name_length(&name);
    println!("名前はまだ使用可能: {}", name);
    
    // 可変参照の受け取り
    let mut number = 42;
    modify_number(&mut number);
    println!("変更後の数値: {}", number);
    
    // ------------------------------------------
    // 高階関数
    // ------------------------------------------
    
    println!("\n=== 高階関数 ===");
    
    // 関数を引数として受け取る
    let numbers = vec![1, 2, 3, 4, 5];
    
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("二乗: {:?}", squared);
    
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);
    
    let sum: i32 = numbers.iter().sum();
    println!("合計: {}", sum);
    
    // 関数ポインタ
    let operation = add;
    let result = operation(5, 3);
    println!("関数ポインタ経由: {}", result);
    
    // apply_operation関数の使用
    let add_result = apply_operation(10, 5, add);
    let multiply_result = apply_operation(10, 5, multiply);
    println!("適用結果 - 加算: {}, 乗算: {}", add_result, multiply_result);
    
    // ------------------------------------------
    // クロージャ
    // ------------------------------------------
    
    println!("\n=== クロージャ ===");
    
    // 基本的なクロージャ
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5));
    
    // 複数の引数を持つクロージャ
    let multiply_closure = |x, y| x * y;
    println!("3 * 4 = {}", multiply_closure(3, 4));
    
    // 環境をキャプチャするクロージャ
    let factor = 10;
    let scale = |x| x * factor; // 外部変数をキャプチャ
    println!("5を{}倍: {}", factor, scale(5));
    
    // moveクロージャ
    let name = String::from("Rust");
    let greeting_closure = move |suffix| {
        format!("Hello, {}{}!", name, suffix) // nameの所有権を移動
    };
    println!("{}", greeting_closure(" World"));
    // name はここではもう使用できない
    
    // クロージャを返す関数
    let multiplier = create_multiplier(3);
    println!("4の3倍: {}", multiplier(4));
    
    // ------------------------------------------
    // ジェネリック関数
    // ------------------------------------------
    
    println!("\n=== ジェネリック関数 ===");
    
    // 型パラメータを持つ関数
    let max_int = max(10, 20);
    let max_float = max(3.14, 2.71);
    let max_char = max('a', 'z');
    
    println!("最大値 - 整数: {}, 浮動小数点: {}, 文字: {}", max_int, max_float, max_char);
    
    // 複数の型パラメータ
    let pair = create_pair(42, "Hello");
    println!("ペア: {:?}", pair);
    
    // トレイト境界付きジェネリック
    let numbers = vec![1, 5, 3, 9, 2];
    let largest = find_largest(&numbers);
    println!("最大値: {}", largest);
    
    // ------------------------------------------
    // 再帰関数
    // ------------------------------------------
    
    println!("\n=== 再帰関数 ===");
    
    let factorial_5 = factorial(5);
    println!("5! = {}", factorial_5);
    
    let fib_10 = fibonacci(10);
    println!("fibonacci(10) = {}", fib_10);
    
    // 相互再帰
    println!("is_even(4): {}", is_even(4));
    println!("is_odd(4): {}", is_odd(4));
    
    // ------------------------------------------
    // エラーを返す関数
    // ------------------------------------------
    
    println!("\n=== エラーを返す関数 ===");
    
    // Result型を返す関数
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(error) => println!("エラー: {}", error),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(error) => println!("エラー: {}", error),
    }
    
    // ? オペレータの使用
    match process_numbers() {
        Ok(result) => println!("処理結果: {}", result),
        Err(error) => println!("処理エラー: {}", error),
    }
    
    // ------------------------------------------
    // メソッド構文
    // ------------------------------------------
    
    println!("\n=== メソッド構文 ===");
    
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("長方形の面積: {}", rect.area());
    println!("長方形の周囲: {}", rect.perimeter());
    
    let mut square = Rectangle::square(25);
    println!("正方形の面積: {}", square.area());
    
    square.scale(2);
    println!("拡大後の正方形の面積: {}", square.area());
}

// ------------------------------------------
// 関数定義
// ------------------------------------------

// 引数なし、返り値なし
fn greet() {
    println!("こんにちは！");
}

// 引数あり、返り値なし
fn greet_person(name: &str) {
    println!("こんにちは、{}さん！", name);
}

// 引数あり、返り値あり
fn add(a: i32, b: i32) -> i32 {
    a + b  // 最後の式が返り値（returnキーワードは省略可能）
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 複数の返り値
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

// 早期返却
fn early_return_example(x: i32) -> i32 {
    if x < 0 {
        return 0; // 早期返却
    }
    
    if x > 100 {
        return 100; // 早期返却
    }
    
    x * 2  // 通常の返却
}

// 構造体定義
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// タプルの分解
fn print_coordinates((x, y): (i32, i32)) {
    println!("座標: ({}, {})", x, y);
}

// 構造体の分解
fn print_person_info(Person { name, age }: Person) {
    println!("人物情報: {} ({}歳)", name, age);
}

// 参照の受け取り
fn print_name_length(name: &String) {
    println!("名前「{}」の長さ: {}", name, name.len());
}

// 可変参照の受け取り
fn modify_number(number: &mut i32) {
    *number *= 2;
}

// 関数を引数として受け取る
fn apply_operation(a: i32, b: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(a, b)
}

// クロージャを返す関数
fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

// ジェネリック関数
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn create_pair<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

// トレイト境界付きジェネリック
fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// 再帰関数
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 相互再帰
fn is_even(n: u32) -> bool {
    match n {
        0 => true,
        _ => is_odd(n - 1),
    }
}

fn is_odd(n: u32) -> bool {
    match n {
        0 => false,
        _ => is_even(n - 1),
    }
}

// エラーを返す関数
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("ゼロで割ることはできません"))
    } else {
        Ok(a / b)
    }
}

// ? オペレータの使用例
fn process_numbers() -> Result<i32, String> {
    let a = parse_number("10")?;
    let b = parse_number("5")?;
    Ok(a + b)
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| format!("'{}' は有効な数値ではありません", s))
}

// メソッド定義
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // メソッド（&selfを受け取る）
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    // 関連関数（Selfを返さない、::で呼び出す）
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // 可変メソッド（&mut selfを受け取る）
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}