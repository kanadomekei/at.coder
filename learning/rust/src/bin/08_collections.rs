use std::collections::HashMap;

fn main() {
    println!("=== Rustのコレクション ===");

    // Vec<T>
    println!("\n=== Vector ===");
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("ベクター: {:?}", v);

    let v2 = vec![1, 2, 3, 4, 5];
    println!("マクロで作成: {:?}", v2);

    // 要素アクセス
    let third: &i32 = &v2[2];
    println!("3番目の要素: {}", third);

    match v2.get(2) {
        Some(third) => println!("3番目の要素: {}", third),
        None => println!("3番目の要素はありません"),
    }

    // 反復処理
    for i in &v {
        println!("値: {}", i);
    }

    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
    }
    println!("変更後: {:?}", v3);

    // HashMap
    println!("\n=== HashMap ===");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("{}チームのスコア: {}", team_name, s),
        None => println!("チームが見つかりません"),
    }

    // 反復処理
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // エントリAPI
    scores.entry(String::from("Red")).or_insert(25);
    scores.entry(String::from("Blue")).or_insert(25); // 既存の値は変更されない
    println!("最終スコア: {:?}", scores);
}
