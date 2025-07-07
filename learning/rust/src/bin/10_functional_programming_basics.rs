// Rust 関数型プログラミング - 基礎編
// 関数型プログラミングの基本概念とRustでの実装

// 1. 関数型プログラミングとは
// - 関数を第一級オブジェクトとして扱う
// - 不変性（Immutability）を重視
// - 副作用を避ける
// - 関数の合成と組み合わせによる問題解決

// 2. 不変性（Immutability）の重要性
#[allow(unused)]
fn immutability_examples() {
    // 不変な値
    let x = 10;
    // x = 20; // コンパイルエラー！
    
    // 可変な値
    let mut y = 10;
    y = 20; // OK
    
    // 不変なベクター
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 関数型的なアプローチ：新しいベクターを作成
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Original: {:?}", numbers);
    println!("Doubled: {:?}", doubled);
    
    // 文字列の不変性
    let s1 = "Hello";
    let s2 = format!("{}, World!", s1);
    println!("s1: {}, s2: {}", s1, s2);
}

// 3. 純粋関数（Pure Functions）
// 副作用がなく、同じ入力に対して常に同じ出力を返す関数

// 純粋関数の例
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 純粋関数：数学的な計算
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// 純粋関数：文字列操作
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// 4. 関数を値として扱う
#[allow(unused)]
fn functions_as_values() {
    // 関数を変数に代入
    let add_func = add;
    let result = add_func(5, 3);
    println!("Result: {}", result);
    
    // 関数を引数として渡す
    fn apply_operation(x: i32, y: i32, op: fn(i32, i32) -> i32) -> i32 {
        op(x, y)
    }
    
    let sum = apply_operation(10, 20, add);
    let product = apply_operation(10, 20, multiply);
    
    println!("Sum: {}", sum);
    println!("Product: {}", product);
}

// 5. 関数型的なデータ変換
#[allow(unused)]
fn functional_data_transformation() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // map: 各要素を変換
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squares: {:?}", squares);
    
    // filter: 条件に合う要素を抽出
    let evens: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();
    println!("Evens: {:?}", evens);
    
    // チェーンングして組み合わせ
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("Even squares: {:?}", even_squares);
}

// 6. fold/reduce パターン
#[allow(unused)]
fn fold_examples() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // fold: 累積計算
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);
    
    // reduce: 最初の要素を初期値として使用
    let product = numbers.iter().cloned().reduce(|acc, x| acc * x);
    println!("Product: {:?}", product);
    
    // 文字列の連結
    let words = vec!["Hello", "functional", "programming"];
    let sentence = words.iter().fold(String::new(), |acc, word| {
        if acc.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", acc, word)
        }
    });
    println!("Sentence: {}", sentence);
}

// 7. Option型と関数型プログラミング
#[allow(unused)]
fn option_functional_style() {
    // Option型は関数型プログラミングの重要な概念
    let maybe_number = Some(42);
    let no_number: Option<i32> = None;
    
    // map: Option内の値を変換
    let doubled = maybe_number.map(|x| x * 2);
    println!("Doubled: {:?}", doubled);
    
    // filter: 条件に合う場合のみ値を保持
    let even_only = maybe_number.filter(|&x| x % 2 == 0);
    println!("Even only: {:?}", even_only);
    
    // and_then: flatMapのような操作
    let result = maybe_number.and_then(|x| {
        if x > 0 {
            Some(x * 2)
        } else {
            None
        }
    });
    println!("Result: {:?}", result);
}

// 8. Result型と関数型プログラミング
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[allow(unused)]
fn result_functional_style() {
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);
    
    // map: 成功時の値を変換
    let doubled = result1.map(|x| x * 2.0);
    println!("Doubled result: {:?}", doubled);
    
    // map_err: エラーを変換
    let custom_error = result2.map_err(|e| format!("Error: {}", e));
    println!("Custom error: {:?}", custom_error);
    
    // and_then: チェーンング
    let chained = divide(20.0, 4.0)
        .and_then(|x| divide(x, 2.0))
        .and_then(|x| divide(x, 1.0));
    println!("Chained result: {:?}", chained);
}

// 9. 関数型的なエラーハンドリング
#[allow(unused)]
fn functional_error_handling() {
    let numbers = vec!["42", "not_a_number", "100", "invalid"];
    
    // collect()でResult<Vec<_>, _>を作成
    let parsed: Result<Vec<i32>, _> = numbers
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    
    match parsed {
        Ok(nums) => println!("All numbers: {:?}", nums),
        Err(e) => println!("Parse error: {}", e),
    }
    
    // filter_map: 成功したもののみを収集
    let valid_numbers: Vec<i32> = numbers
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("Valid numbers: {:?}", valid_numbers);
}

// 10. 実践的な関数型プログラミングの例
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[allow(unused)]
fn practical_functional_example() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
        Person { name: "David".to_string(), age: 20 },
    ];
    
    // 関数型的なデータ処理
    let adult_names: Vec<String> = people
        .iter()
        .filter(|person| person.age >= 21)
        .map(|person| person.name.clone())
        .collect();
    
    println!("Adults: {:?}", adult_names);
    
    // 年齢の合計
    let total_age: u32 = people.iter().map(|p| p.age).sum();
    println!("Total age: {}", total_age);
    
    // 最高齢者を見つける
    let oldest = people.iter().max_by_key(|p| p.age);
    println!("Oldest person: {:?}", oldest);
}

// メイン関数
fn main() {
    println!("=== Rust 関数型プログラミング 基礎編 ===\n");
    
    println!("1. 不変性の例:");
    immutability_examples();
    
    println!("\n2. 純粋関数の例:");
    println!("add(5, 3) = {}", add(5, 3));
    println!("factorial(5) = {}", factorial(5));
    println!("reverse_string(\"hello\") = {}", reverse_string("hello"));
    
    println!("\n3. 関数を値として扱う:");
    functions_as_values();
    
    println!("\n4. 関数型的なデータ変換:");
    functional_data_transformation();
    
    println!("\n5. fold/reduce パターン:");
    fold_examples();
    
    println!("\n6. Option型と関数型プログラミング:");
    option_functional_style();
    
    println!("\n7. Result型と関数型プログラミング:");
    result_functional_style();
    
    println!("\n8. 関数型的なエラーハンドリング:");
    functional_error_handling();
    
    println!("\n9. 実践的な関数型プログラミング:");
    practical_functional_example();
}

// 練習問題
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pure_functions() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(multiply(4, 5), 20);
        assert_eq!(factorial(4), 24);
        assert_eq!(reverse_string("abc"), "cba");
    }
    
    #[test]
    fn test_functional_transformations() {
        let numbers = vec![1, 2, 3, 4, 5];
        
        let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
        assert_eq!(squares, vec![1, 4, 9, 16, 25]);
        
        let evens: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();
        assert_eq!(evens, vec![2, 4]);
    }
    
    #[test]
    fn test_option_operations() {
        let some_value = Some(10);
        let none_value: Option<i32> = None;
        
        assert_eq!(some_value.map(|x| x * 2), Some(20));
        assert_eq!(none_value.map(|x| x * 2), None);
        
        assert_eq!(some_value.filter(|&x| x > 5), Some(10));
        assert_eq!(some_value.filter(|&x| x > 15), None);
    }
}