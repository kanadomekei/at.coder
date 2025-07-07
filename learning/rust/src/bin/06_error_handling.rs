fn main() {
    // ==========================================
    // エラーハンドリング
    // ==========================================
    
    println!("=== Rustのエラーハンドリング ===");
    
    // ------------------------------------------
    // パニックによる回復不可能なエラー
    // ------------------------------------------
    
    println!("\n=== パニック ===");
    
    // パニックの例（実際にはコメントアウト）
    // panic!("クラッシュして燃える");
    
    // 境界外アクセスによるパニック（コメントアウト）
    // let v = vec![1, 2, 3];
    // v[99]; // パニックが発生
    
    println!("パニックは回復不可能なエラーです");
    
    // ------------------------------------------
    // Result型による回復可能なエラー
    // ------------------------------------------
    
    println!("\n=== Result型 ===");
    
    // ファイル操作のシミュレーション
    match read_file("hello.txt") {
        Ok(contents) => println!("ファイル内容: {}", contents),
        Err(error) => println!("ファイル読み込みエラー: {}", error),
    }
    
    match read_file("missing.txt") {
        Ok(contents) => println!("ファイル内容: {}", contents),
        Err(error) => println!("ファイル読み込みエラー: {}", error),
    }
    
    // ------------------------------------------
    // matchによるエラーハンドリング
    // ------------------------------------------
    
    println!("\n=== matchによるエラーハンドリング ===");
    
    let result = divide(10.0, 2.0);
    match result {
        Ok(value) => println!("除算結果: {}", value),
        Err(error) => match error {
            MathError::DivisionByZero => println!("エラー: ゼロ除算"),
            MathError::NegativeNumber => println!("エラー: 負の数"),
            MathError::Overflow => println!("エラー: オーバーフロー"),
        },
    }
    
    let result = divide(10.0, 0.0);
    match result {
        Ok(value) => println!("除算結果: {}", value),
        Err(error) => match error {
            MathError::DivisionByZero => println!("エラー: ゼロ除算"),
            MathError::NegativeNumber => println!("エラー: 負の数"),
            MathError::Overflow => println!("エラー: オーバーフロー"),
        },
    }
    
    // ------------------------------------------
    // unwrapとexpect
    // ------------------------------------------
    
    println!("\n=== unwrapとexpect ===");
    
    // 成功する場合
    let result = divide(10.0, 2.0);
    let value = result.unwrap(); // パニックしない
    println!("unwrapした値: {}", value);
    
    // expectを使用
    let result = divide(15.0, 3.0);
    let value = result.expect("除算に失敗しました"); // カスタムメッセージ
    println!("expectした値: {}", value);
    
    // エラーの場合（コメントアウト）
    // let result = divide(10.0, 0.0);
    // let value = result.unwrap(); // パニックする
    
    // ------------------------------------------
    // ? オペレータ
    // ------------------------------------------
    
    println!("\n=== ? オペレータ ===");
    
    match calculate_sum("5", "10") {
        Ok(sum) => println!("合計: {}", sum),
        Err(error) => println!("計算エラー: {}", error),
    }
    
    match calculate_sum("5", "abc") {
        Ok(sum) => println!("合計: {}", sum),
        Err(error) => println!("計算エラー: {}", error),
    }
    
    // チェーンされた ? オペレータ
    match complex_calculation() {
        Ok(result) => println!("複雑な計算結果: {}", result),
        Err(error) => println!("複雑な計算エラー: {}", error),
    }
    
    // ------------------------------------------
    // カスタムエラー型
    // ------------------------------------------
    
    println!("\n=== カスタムエラー型 ===");
    
    match validate_user_data("Alice", 25, "alice@example.com") {
        Ok(user) => println!("ユーザー作成成功: {:?}", user),
        Err(error) => println!("バリデーションエラー: {}", error),
    }
    
    match validate_user_data("", 25, "alice@example.com") {
        Ok(user) => println!("ユーザー作成成功: {:?}", user),
        Err(error) => println!("バリデーションエラー: {}", error),
    }
    
    match validate_user_data("Bob", 15, "bob@example.com") {
        Ok(user) => println!("ユーザー作成成功: {:?}", user),
        Err(error) => println!("バリデーションエラー: {}", error),
    }
    
    // ------------------------------------------
    // エラーの伝播
    // ------------------------------------------
    
    println!("\n=== エラーの伝播 ===");
    
    match process_config_file() {
        Ok(config) => println!("設定読み込み成功: {:?}", config),
        Err(error) => println!("設定読み込みエラー: {}", error),
    }
    
    // ------------------------------------------
    // Result型のメソッド
    // ------------------------------------------
    
    println!("\n=== Result型のメソッド ===");
    
    let success: Result<i32, String> = Ok(42);
    let error: Result<i32, String> = Err(String::from("エラー"));
    
    // map メソッド
    let doubled = success.clone().map(|x| x * 2);
    println!("map結果: {:?}", doubled);
    
    // map_err メソッド
    let detailed_error = error.clone().map_err(|e| format!("詳細エラー: {}", e));
    println!("map_err結果: {:?}", detailed_error);
    
    // and_then メソッド
    let chained = success.and_then(|x| if x > 0 { Ok(x * 2) } else { Err(String::from("負の値")) });
    println!("and_then結果: {:?}", chained);
    
    // or_else メソッド
    let fallback: Result<i32, String> = error.clone().or_else(|_| Ok(0));
    println!("or_else結果: {:?}", fallback);
    
    // unwrap_or メソッド
    let value_or_default = error.clone().unwrap_or(-1);
    println!("unwrap_or結果: {}", value_or_default);
    
    // unwrap_or_else メソッド
    let computed_default = error.unwrap_or_else(|e| {
        println!("エラーが発生: {}", e);
        -1
    });
    println!("unwrap_or_else結果: {}", computed_default);
    
    // ------------------------------------------
    // エラーのチェーン
    // ------------------------------------------
    
    println!("\n=== エラーのチェーン ===");
    
    // anyhow クレートのようなエラーチェーンのシミュレーション
    match process_data_pipeline() {
        Ok(result) => println!("パイプライン成功: {}", result),
        Err(error) => {
            println!("パイプラインエラー: {}", error);
            // 実際のanyhowクレートでは error.chain() でエラーチェーンを辿れる
        }
    }
    
    // ------------------------------------------
    // パニック時の対処
    // ------------------------------------------
    
    println!("\n=== パニック時の対処 ===");
    
    // catch_unwind を使ったパニックのキャッチ（推奨されない）
    use std::panic;
    
    let result = panic::catch_unwind(|| {
        // パニックが発生する可能性のあるコード
        safe_operation(5)
    });
    
    match result {
        Ok(value) => println!("安全な操作成功: {}", value),
        Err(_) => println!("パニックがキャッチされました"),
    }
    
    // パニックフックの設定
    panic::set_hook(Box::new(|panic_info| {
        println!("カスタムパニックハンドラ: {:?}", panic_info);
    }));
    
    // User構造体とConfig構造体の使用例
    println!("\n=== 構造体の使用例 ===");
    let user = User {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };
    
    let config = Config {
        database_url: String::from("localhost:5432"),
        timeout: 30,
    };
    
    println!("ユーザー: {} ({}歳) - {}", user.name, user.age, user.email);
    println!("設定: DB URL: {}, タイムアウト: {}秒", config.database_url, config.timeout);
}

// ------------------------------------------
// エラー型の定義
// ------------------------------------------

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeNumber,
    Overflow,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "ゼロで割ることはできません"),
            MathError::NegativeNumber => write!(f, "負の数は処理できません"),
            MathError::Overflow => write!(f, "計算結果がオーバーフローしました"),
        }
    }
}

impl std::error::Error for MathError {}

// バリデーションエラー
#[derive(Debug)]
enum ValidationError {
    EmptyName,
    InvalidAge,
    InvalidEmail,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::EmptyName => write!(f, "名前が空です"),
            ValidationError::InvalidAge => write!(f, "無効な年齢です"),
            ValidationError::InvalidEmail => write!(f, "無効なメールアドレスです"),
        }
    }
}

impl std::error::Error for ValidationError {}

// ユーザー構造体
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

// 設定構造体
#[derive(Debug)]
struct Config {
    database_url: String,
    timeout: u32,
}

// ------------------------------------------
// 関数定義
// ------------------------------------------

fn read_file(filename: &str) -> Result<String, String> {
    match filename {
        "hello.txt" => Ok(String::from("Hello, World!")),
        _ => Err(String::from("ファイルが見つかりません")),
    }
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else if a < 0.0 || b < 0.0 {
        Err(MathError::NegativeNumber)
    } else {
        let result = a / b;
        if result.is_infinite() {
            Err(MathError::Overflow)
        } else {
            Ok(result)
        }
    }
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| format!("'{}' は有効な数値ではありません", s))
}

fn calculate_sum(a: &str, b: &str) -> Result<i32, String> {
    let num_a = parse_number(a)?; // ? オペレータでエラーを伝播
    let num_b = parse_number(b)?;
    Ok(num_a + num_b)
}

fn complex_calculation() -> Result<i32, String> {
    let a = parse_number("10")?;
    let b = parse_number("20")?;
    let c = parse_number("30")?;
    
    // より複雑な計算
    let intermediate = (a + b) * c;
    if intermediate > 1000 {
        Err(String::from("結果が大きすぎます"))
    } else {
        Ok(intermediate)
    }
}

fn validate_user_data(name: &str, age: u32, email: &str) -> Result<User, ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::EmptyName);
    }
    
    if age < 18 || age > 120 {
        return Err(ValidationError::InvalidAge);
    }
    
    if !email.contains('@') {
        return Err(ValidationError::InvalidEmail);
    }
    
    Ok(User {
        name: name.to_string(),
        age,
        email: email.to_string(),
    })
}

fn read_config() -> Result<String, String> {
    // 設定ファイルの読み込みをシミュレート
    Ok(String::from("database_url=localhost;timeout=30"))
}

fn parse_config(content: &str) -> Result<Config, String> {
    // 簡単なパースをシミュレート
    if content.contains("database_url") && content.contains("timeout") {
        Ok(Config {
            database_url: String::from("localhost"),
            timeout: 30,
        })
    } else {
        Err(String::from("設定の形式が正しくありません"))
    }
}

fn process_config_file() -> Result<Config, String> {
    let content = read_config()?;
    let config = parse_config(&content)?;
    Ok(config)
}

fn process_data_pipeline() -> Result<String, String> {
    // 複数の処理ステップをシミュレート
    let data = read_file("data.txt")?;
    let processed = format!("Processed: {}", data);
    
    if processed.len() > 100 {
        Err(String::from("処理されたデータが大きすぎます"))
    } else {
        Ok(processed)
    }
}

fn safe_operation(x: i32) -> i32 {
    if x < 0 {
        panic!("負の値は許可されていません");
    }
    x * 2
}