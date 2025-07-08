fn main() {
    // ==========================================
    // 変数とデータ型
    // ==========================================

    println!("=== Rustの変数とデータ型 ===");

    // ------------------------------------------
    // 変数の宣言
    // ------------------------------------------

    println!("\n=== 変数の宣言 ===");

    // let: 不変変数（デフォルト）
    let name = "Alice";
    println!("不変変数 name: {}", name);

    // let mut: 可変変数
    let mut counter = 0;
    println!("初期値 counter: {}", counter);
    counter += 10;
    println!("変更後 counter: {}", counter);

    // 型注釈
    let age: u32 = 25;
    let height: f64 = 165.5;
    println!("年齢: {}, 身長: {}", age, height);

    // 変数のシャドーイング
    let x = 5;
    let x = x + 1; // 新しい変数として再定義
    let x = x * 2;
    println!("シャドーイングされた x: {}", x);

    // 型を変更するシャドーイング
    let spaces = "   ";
    let spaces = spaces.len(); // 文字列から数値に型変更
    println!("スペースの数: {}", spaces);

    // ------------------------------------------
    // 整数型
    // ------------------------------------------

    println!("\n=== 整数型 ===");

    // 符号付き整数
    let i8_val: i8 = -128; // 8bit: -128 to 127
    let i16_val: i16 = -32768; // 16bit: -32768 to 32767
    let i32_val: i32 = -2147483648; // 32bit (デフォルト)
    let i64_val: i64 = -9223372036854775808; // 64bit
    let i128_val: i128 = -170141183460469231731687303715884105728; // 128bit
    let isize_val: isize = -100; // ポインタサイズ

    println!("符号付き整数:");
    println!("i8: {}, i16: {}, i32: {}", i8_val, i16_val, i32_val);
    println!("i64: {}, i128: {}, isize: {}", i64_val, i128_val, isize_val);

    // 符号なし整数
    let u8_val: u8 = 255; // 8bit: 0 to 255
    let u16_val: u16 = 65535; // 16bit: 0 to 65535
    let u32_val: u32 = 4294967295; // 32bit
    let u64_val: u64 = 18446744073709551615; // 64bit
    let u128_val: u128 = 340282366920938463463374607431768211455; // 128bit
    let usize_val: usize = 100; // ポインタサイズ

    println!("符号なし整数:");
    println!("u8: {}, u16: {}, u32: {}", u8_val, u16_val, u32_val);
    println!("u64: {}, u128: {}, usize: {}", u64_val, u128_val, usize_val);

    // 数値リテラルの記法
    let decimal = 98_222; // 10進数（アンダースコアで区切り可能）
    let hex = 0xff; // 16進数
    let octal = 0o77; // 8進数
    let binary = 0b1111_0000; // 2進数
    let byte = b'A'; // バイトリテラル（u8のみ）

    println!("数値リテラル:");
    println!(
        "decimal: {}, hex: {}, octal: {}, binary: {}, byte: {}",
        decimal, hex, octal, binary, byte
    );

    // ------------------------------------------
    // 浮動小数点型
    // ------------------------------------------

    println!("\n=== 浮動小数点型 ===");

    let f32_val: f32 = 3.14159; // 32bit浮動小数点
    let f64_val: f64 = 2.718281828459045; // 64bit浮動小数点（デフォルト）

    println!("f32: {}, f64: {}", f32_val, f64_val);

    // 科学記法
    let scientific1 = 1e6; // 1,000,000
    let scientific2 = 2.5e-3; // 0.0025

    println!("科学記法: 1e6 = {}, 2.5e-3 = {}", scientific1, scientific2);

    // ------------------------------------------
    // 論理型
    // ------------------------------------------

    println!("\n=== 論理型 ===");

    let is_rust_awesome: bool = true;
    let is_learning: bool = false;

    println!("Rustは素晴らしい: {}", is_rust_awesome);
    println!("学習中: {}", is_learning);

    // 論理演算
    let and_result = is_rust_awesome && is_learning;
    let or_result = is_rust_awesome || is_learning;
    let not_result = !is_rust_awesome;

    println!(
        "AND: {}, OR: {}, NOT: {}",
        and_result, or_result, not_result
    );

    // ------------------------------------------
    // 文字型
    // ------------------------------------------

    println!("\n=== 文字型 ===");

    let char_a: char = 'A';
    let char_unicode: char = '🦀'; // Rustのマスコット（カニ）
    let char_japanese: char = 'あ';
    let char_emoji: char = '😊';

    println!(
        "文字: {}, Unicode: {}, 日本語: {}, 絵文字: {}",
        char_a, char_unicode, char_japanese, char_emoji
    );

    // ------------------------------------------
    // 文字列型
    // ------------------------------------------

    println!("\n=== 文字列型 ===");

    // 文字列スライス（&str）
    let string_slice: &str = "Hello, Rust!";
    let literal = "これは文字列リテラルです";

    println!("文字列スライス: {}", string_slice);
    println!("リテラル: {}", literal);

    // String型（所有権を持つ文字列）
    let mut owned_string = String::from("Hello");
    owned_string.push_str(", World!");

    println!("所有権を持つ文字列: {}", owned_string);

    // rawストリング
    let raw_string = r"このraw文字列では\nや\tをエスケープしません";
    let raw_multiline = r#"
        複数行のraw文字列
        "引用符"も使えます
    "#;

    println!("Raw文字列: {}", raw_string);
    println!("Raw複数行: {}", raw_multiline);

    // ------------------------------------------
    // タプル型
    // ------------------------------------------

    println!("\n=== タプル型 ===");

    let person: (String, u32, f64) = (String::from("Bob"), 30, 175.5);
    println!("人物タプル: {:?}", person);

    // タプルの分解
    let (name, age, height) = person;
    println!("名前: {}, 年齢: {}, 身長: {}", name, age, height);

    // インデックスによるアクセス
    let coordinates = (10.5, 20.3);
    println!("座標: x = {}, y = {}", coordinates.0, coordinates.1);

    // 空のタプル（unit型）
    let unit: () = ();
    println!("Unit型: {:?}", unit);

    // ------------------------------------------
    // 配列型
    // ------------------------------------------

    println!("\n=== 配列型 ===");

    // 固定長配列
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 3]; // [0, 0, 0]

    println!("数値配列: {:?}", numbers);
    println!("ゼロ配列: {:?}", zeros);
    println!("配列の長さ: {}", numbers.len());
    println!("最初の要素: {}", numbers[0]);

    // 配列のスライス
    let slice = &numbers[1..4]; // インデックス1から3まで
    println!("スライス: {:?}", slice);

    // ------------------------------------------
    // 型変換
    // ------------------------------------------

    println!("\n=== 型変換 ===");

    // as キーワードによる型変換
    let integer = 42i32;
    let float_from_int = integer as f64;

    println!("整数から浮動小数点: {} -> {}", integer, float_from_int);

    // 文字列から数値への変換
    let string_number = "42";
    let parsed_number: i32 = string_number.parse().unwrap();

    println!("文字列から数値: \"{}\" -> {}", string_number, parsed_number);

    // 数値から文字列への変換
    let number = 123;
    let number_string = number.to_string();

    println!("数値から文字列: {} -> \"{}\"", number, number_string);

    // ------------------------------------------
    // 定数
    // ------------------------------------------

    println!("\n=== 定数 ===");

    const MAX_POINTS: u32 = 100_000;
    const PI: f64 = 3.14159265359;

    println!("最大ポイント: {}", MAX_POINTS);
    println!("円周率: {}", PI);

    // ------------------------------------------
    // Option型とResult型（基本）
    // ------------------------------------------

    println!("\n=== Option型とResult型（基本） ===");

    // Option型
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    match some_number {
        Some(value) => println!("値があります: {}", value),
        None => println!("値がありません"),
    }

    match no_number {
        Some(value) => println!("値があります: {}", value),
        None => println!("値がありません"),
    }

    // Result型
    let division_result: Result<i32, &str> = divide(10, 2);
    match division_result {
        Ok(value) => println!("除算結果: {}", value),
        Err(error) => println!("エラー: {}", error),
    }

    let error_result: Result<i32, &str> = divide(10, 0);
    match error_result {
        Ok(value) => println!("除算結果: {}", value),
        Err(error) => println!("エラー: {}", error),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("ゼロで割ることはできません")
    } else {
        Ok(a / b)
    }
}
