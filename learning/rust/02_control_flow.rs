fn main() {
    // ==========================================
    // 制御構文
    // ==========================================
    
    println!("=== Rustの制御構文 ===");
    
    // ------------------------------------------
    // if文
    // ------------------------------------------
    
    println!("\n=== if文 ===");
    
    let score = 85;
    
    // 基本的なif文
    if score >= 90 {
        println!("優秀です！");
    } else if score >= 70 {
        println!("良い成績です。");
    } else if score >= 50 {
        println!("合格です。");
    } else {
        println!("がんばりましょう。");
    }
    
    // if式（値を返す）
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else {
        "D"
    };
    println!("成績: {}", grade);
    
    // 複雑な条件
    let temperature = 25;
    let is_sunny = true;
    
    if temperature > 20 && is_sunny {
        println!("散歩にいい天気です！");
    } else if temperature > 20 || is_sunny {
        println!("まあまあの天気です。");
    } else {
        println!("家にいましょう。");
    }
    
    // ------------------------------------------
    // match文
    // ------------------------------------------
    
    println!("\n=== match文 ===");
    
    let day = 3;
    
    // 基本的なmatch文
    match day {
        1 => println!("月曜日"),
        2 => println!("火曜日"),
        3 => println!("水曜日"),
        4 => println!("木曜日"),
        5 => println!("金曜日"),
        6 | 7 => println!("週末"), // 複数の値をまとめて処理
        _ => println!("無効な日付"), // その他すべて
    }
    
    // match式（値を返す）
    let day_type = match day {
        1..=5 => "平日",     // 範囲指定
        6 | 7 => "週末",
        _ => "無効",
    };
    println!("日付タイプ: {}", day_type);
    
    // Option型のmatch
    let maybe_number: Option<i32> = Some(42);
    
    match maybe_number {
        Some(value) => println!("値があります: {}", value),
        None => println!("値がありません"),
    }
    
    // Result型のmatch
    let division_result: Result<i32, String> = divide_safe(10, 2);
    
    match division_result {
        Ok(value) => println!("除算結果: {}", value),
        Err(error) => println!("エラー: {}", error),
    }
    
    // 複雑なパターンマッチング
    let coordinates = (0, -2);
    
    match coordinates {
        (0, 0) => println!("原点です"),
        (0, y) => println!("Y軸上の点: y = {}", y),
        (x, 0) => println!("X軸上の点: x = {}", x),
        (x, y) if x == y => println!("対角線上の点: ({}, {})", x, y),
        (x, y) => println!("一般的な点: ({}, {})", x, y),
    }
    
    // ------------------------------------------
    // loop文
    // ------------------------------------------
    
    println!("\n=== loop文 ===");
    
    // 無限ループ（breakで抜ける）
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 3 {
            println!("カウンター: {}", counter);
            continue; // 次の反復へ
        }
        
        if counter == 5 {
            break counter * 2; // 値を返してループを抜ける
        }
        
        println!("カウンター: {}", counter);
    };
    
    println!("ループの結果: {}", result);
    
    // ネストしたループとラベル
    'outer: loop {
        println!("外側のループに入りました");
        
        loop {
            println!("内側のループに入りました");
            break 'outer; // 外側のループを抜ける
        }
    }
    println!("外側のループを抜けました");
    
    // ------------------------------------------
    // while文
    // ------------------------------------------
    
    println!("\n=== while文 ===");
    
    // 基本的なwhile文
    let mut number = 3;
    
    while number != 0 {
        println!("カウントダウン: {}", number);
        number -= 1;
    }
    println!("発射！");
    
    // 条件付きwhile
    let mut stack = vec![1, 2, 3, 4, 5];
    
    while let Some(top) = stack.pop() {
        println!("スタックからポップ: {}", top);
    }
    
    // ------------------------------------------
    // for文
    // ------------------------------------------
    
    println!("\n=== for文 ===");
    
    // 配列に対するfor文
    let fruits = ["apple", "banana", "cherry", "date"];
    
    println!("果物リスト:");
    for fruit in fruits.iter() {
        println!("- {}", fruit);
    }
    
    // インデックス付きfor文
    println!("インデックス付き:");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", index, fruit);
    }
    
    // 範囲for文
    println!("1から5まで:");
    for number in 1..=5 {
        println!("{}", number);
    }
    
    // 逆順
    println!("5から1まで:");
    for number in (1..=5).rev() {
        println!("{}", number);
    }
    
    // ステップ付き
    println!("2ずつ増加:");
    for number in (0..10).step_by(2) {
        println!("{}", number);
    }
    
    // Vecに対するfor文
    let numbers = vec![10, 20, 30, 40, 50];
    
    println!("ベクターの要素:");
    for number in &numbers {
        println!("{}", number);
    }
    
    // 可変参照でfor文
    let mut mutable_numbers = vec![1, 2, 3, 4, 5];
    
    println!("変更前: {:?}", mutable_numbers);
    for number in &mut mutable_numbers {
        *number *= 2;
    }
    println!("変更後: {:?}", mutable_numbers);
    
    // 所有権を移動するfor文
    let owned_numbers = vec![100, 200, 300];
    
    println!("所有権を移動:");
    for number in owned_numbers {
        println!("{}", number);
    }
    // owned_numbers はここではもう使用できない
    
    // ------------------------------------------
    // if let と while let
    // ------------------------------------------
    
    println!("\n=== if let と while let ===");
    
    // if let - Option型の簡潔な処理
    let some_value = Some(3);
    
    if let Some(x) = some_value {
        println!("値を取得しました: {}", x);
    } else {
        println!("値がありませんでした");
    }
    
    // while let - 連続的な処理
    let mut optional_numbers = vec![Some(1), Some(2), Some(3), None, Some(4)];
    
    while let Some(maybe_number) = optional_numbers.pop() {
        match maybe_number {
            Some(number) => println!("数値: {}", number),
            None => {
                println!("Noneが見つかりました");
                break;
            }
        }
    }
    
    // ------------------------------------------
    // 複雑な制御フロー
    // ------------------------------------------
    
    println!("\n=== 複雑な制御フロー ===");
    
    // ネストしたmatch
    let result: Result<Option<i32>, String> = Ok(Some(42));
    
    match result {
        Ok(Some(value)) => println!("成功して値があります: {}", value),
        Ok(None) => println!("成功しましたが値がありません"),
        Err(error) => println!("エラーが発生しました: {}", error),
    }
    
    // パターンガード付きmatch
    let pair = (2, -2);
    
    match pair {
        (x, y) if x == -y => println!("対称的な値です: {} と {}", x, y),
        (x, y) if x > y => println!("最初の値が大きいです: {} > {}", x, y),
        (x, y) if x < y => println!("二番目の値が大きいです: {} < {}", x, y),
        (x, y) => println!("等しい値です: {} = {}", x, y),
    }
    
    // 構造体のパターンマッチング
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let point = Point { x: 0, y: 7 };
    
    match point {
        Point { x, y: 0 } => println!("X軸上の点: x = {}", x),
        Point { x: 0, y } => println!("Y軸上の点: y = {}", y),
        Point { x, y } => println!("その他の点: ({}, {})", x, y),
    }
    
    // 値の束縛
    let message = Some(String::from("Hello"));
    
    match message {
        Some(ref text) if text.len() > 3 => {
            println!("長いメッセージ: {}", text);
        }
        Some(text) => {
            println!("短いメッセージ: {}", text);
        }
        None => println!("メッセージがありません"),
    }
}

fn divide_safe(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("ゼロで割ることはできません"))
    } else {
        Ok(a / b)
    }
}