fn main() {
    // ==========================================
    // 構造体とenum
    // ==========================================
    
    println!("=== Rustの構造体とenum ===");
    
    // ------------------------------------------
    // 基本的な構造体
    // ------------------------------------------
    
    println!("\n=== 基本的な構造体 ===");
    
    // 構造体のインスタンス作成
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
        active: true,
    };
    
    println!("名前: {}, 年齢: {}, メール: {}, アクティブ: {}", 
             person.name, person.age, person.email, person.active);
    
    // フィールド初期化省略記法
    let name = String::from("Bob");
    let email = String::from("bob@example.com");
    let bob = Person {
        name,
        email,
        age: 25,
        active: true,
    };
    
    // 構造体更新記法
    let charlie = Person {
        name: String::from("Charlie"),
        email: String::from("charlie@example.com"),
        ..bob // bobの残りのフィールドをコピー
    };
    
    println!("Charlie: 年齢 {}, アクティブ: {}", charlie.age, charlie.active);
    
    // ------------------------------------------
    // タプル構造体
    // ------------------------------------------
    
    println!("\n=== タプル構造体 ===");
    
    // 色を表すタプル構造体
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("黒色: RGB({}, {}, {})", black.0, black.1, black.2);
    println!("原点: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // 単一フィールドのタプル構造体（newtype pattern）
    let user_id = UserId(12345);
    let product_id = ProductId(67890);
    
    println!("ユーザーID: {}, 商品ID: {}", user_id.0, product_id.0);
    
    // ------------------------------------------
    // Unit-like構造体
    // ------------------------------------------
    
    println!("\n=== Unit-like構造体 ===");
    
    let subject = AlwaysEqual;
    let another_subject = AlwaysEqual;
    
    println!("Unit-like構造体は主にトレイト実装に使用されます");
    
    // ------------------------------------------
    // 構造体のメソッド
    // ------------------------------------------
    
    println!("\n=== 構造体のメソッド ===");
    
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("長方形の面積: {}", rect.area());
    println!("長方形が正方形かどうか: {}", rect.is_square());
    
    // 他の長方形と比較
    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };
    
    println!("rect1にrect2が収まるか: {}", rect.can_hold(&rect2));
    
    // 関連関数を使用
    let square = Rectangle::square(25);
    println!("正方形の面積: {}", square.area());
    
    // ------------------------------------------
    // 基本的なenum
    // ------------------------------------------
    
    println!("\n=== 基本的なenum ===");
    
    let home = IpAddrKind::V4;
    let loopback = IpAddrKind::V6;
    
    route(home);
    route(loopback);
    
    // データを持つenum
    let home_addr = IpAddr::V4(127, 0, 0, 1);
    let loopback_addr = IpAddr::V6(String::from("::1"));
    
    print_ip_addr(home_addr);
    print_ip_addr(loopback_addr);
    
    // ------------------------------------------
    // 複雑なenum
    // ------------------------------------------
    
    println!("\n=== 複雑なenum ===");
    
    // さまざまな型のデータを持つenum
    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write_msg = Message::Write(String::from("Hello"));
    let change_color = Message::ChangeColor(255, 0, 0);
    
    process_message(quit);
    process_message(move_msg);
    process_message(write_msg);
    process_message(change_color);
    
    // ------------------------------------------
    // Option型
    // ------------------------------------------
    
    println!("\n=== Option型 ===");
    
    let some_number = Some(5);
    let some_string = Some("文字列");
    let absent_number: Option<i32> = None;
    
    println!("値のあるOption: {:?}", some_number);
    println!("文字列のOption: {:?}", some_string);
    println!("値のないOption: {:?}", absent_number);
    
    // Option型の操作
    if let Some(value) = some_number {
        println!("値を取得: {}", value);
    }
    
    // map メソッド
    let doubled = some_number.map(|x| x * 2);
    println!("2倍にした値: {:?}", doubled);
    
    // and_then メソッド
    let result = some_number.and_then(|x| if x > 0 { Some(x * 2) } else { None });
    println!("条件付き変換: {:?}", result);
    
    // unwrap_or メソッド
    let value = absent_number.unwrap_or(0);
    println!("デフォルト値を使用: {}", value);
    
    // ------------------------------------------
    // Result型
    // ------------------------------------------
    
    println!("\n=== Result型 ===");
    
    // 成功ケース
    match divide(10.0, 2.0) {
        Ok(result) => println!("除算成功: {}", result),
        Err(error) => println!("除算エラー: {}", error),
    }
    
    // エラーケース
    match divide(10.0, 0.0) {
        Ok(result) => println!("除算成功: {}", result),
        Err(error) => println!("除算エラー: {}", error),
    }
    
    // Result型の操作
    let success_result: Result<i32, String> = Ok(42);
    let error_result: Result<i32, String> = Err(String::from("エラー"));
    
    // map メソッド
    let mapped_success = success_result.map(|x| x * 2);
    println!("成功の場合の変換: {:?}", mapped_success);
    
    // map_err メソッド
    let mapped_error = error_result.map_err(|e| format!("詳細: {}", e));
    println!("エラーの場合の変換: {:?}", mapped_error);
    
    // ------------------------------------------
    // パターンマッチング
    // ------------------------------------------
    
    println!("\n=== パターンマッチング ===");
    
    // 列挙型のパターンマッチング
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("コインの価値: {}セント", value);
    
    // Option型とパターンマッチング
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("5 + 1 = {:?}", six);
    println!("None + 1 = {:?}", none);
    
    // プレースホルダーパターン
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    
    // _プレースホルダー
    let dice_roll = 5;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // その他は何もしない
    }
    
    // ------------------------------------------
    // if let制御フロー
    // ------------------------------------------
    
    println!("\n=== if let制御フロー ===");
    
    let config_max = Some(3u8);
    
    // match を使った場合
    match config_max {
        Some(max) => println!("最大値は {} です", max),
        _ => (),
    }
    
    // if let を使った場合（より簡潔）
    if let Some(max) = config_max {
        println!("最大値は {} です", max);
    }
    
    // if let with else
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    
    if let Coin::Quarter(state) = coin {
        println!("州のクォーター: {:?}", state);
    } else {
        count += 1;
    }
    
    // ------------------------------------------
    // ジェネリック構造体
    // ------------------------------------------
    
    println!("\n=== ジェネリック構造体 ===");
    
    let integer_point = GenericPoint { x: 5, y: 10 };
    let float_point = GenericPoint { x: 1.0, y: 4.0 };
    let mixed_point = MixedPoint { x: 5, y: 4.0 };
    
    println!("整数ポイント: x = {}, y = {}", integer_point.x, integer_point.y);
    println!("浮動小数点ポイント: x = {}, y = {}", float_point.x, float_point.y);
    println!("混合ポイント: x = {}, y = {}", mixed_point.x, mixed_point.y);
    
    // ジェネリックメソッド
    let p1 = GenericPoint { x: 5, y: 10.4 };
    let p2 = GenericPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    
    println!("混合結果: x = {}, y = {}", p3.x, p3.y);
}

// ------------------------------------------
// 構造体定義
// ------------------------------------------

// 基本的な構造体
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

// タプル構造体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// newtype pattern
struct UserId(u32);
struct ProductId(u32);

// Unit-like構造体
struct AlwaysEqual;

// メソッドを持つ構造体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // メソッド
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 関連関数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// ------------------------------------------
// enum定義
// ------------------------------------------

// 基本的なenum
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// データを持つenum
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 複雑なenum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// メソッドを持つenum
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitメッセージ"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

// パターンマッチング用のenum
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... 他の州
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// ジェネリック構造体
#[derive(Debug)]
struct GenericPoint<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T> GenericPoint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> GenericPoint<T, U> {
    fn mixup<V, W>(self, other: GenericPoint<V, W>) -> GenericPoint<T, W> {
        GenericPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// ------------------------------------------
// 関数
// ------------------------------------------

fn route(ip_kind: IpAddrKind) {
    println!("IPアドレスの種類: {:?}", ip_kind);
}

fn print_ip_addr(ip: IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
}

fn process_message(msg: Message) {
    msg.call();
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("ゼロで割ることはできません"))
    } else {
        Ok(a / b)
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("州のクォーター: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("おしゃれな帽子を追加");
}

fn remove_fancy_hat() {
    println!("おしゃれな帽子を削除");
}

fn move_player(num_spaces: u8) {
    println!("プレイヤーを {}マス移動", num_spaces);
}