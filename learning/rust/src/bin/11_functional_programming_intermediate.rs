// Rust 関数型プログラミング - 中級編
// 高階関数、クロージャ、イテレータの詳細な学習

// 1. 高階関数（Higher-Order Functions）
// 関数を引数として受け取る、または関数を返す関数

// 基本的な高階関数の例
fn apply_twice<T>(f: fn(T) -> T, x: T) -> T {
    f(f(x))
}

fn square(x: i32) -> i32 {
    x * x
}

fn increment(x: i32) -> i32 {
    x + 1
}

// より複雑な高階関数
fn compose<A, B, C>(f: fn(B) -> C, g: fn(A) -> B) -> impl Fn(A) -> C {
    move |x| f(g(x))
}

// 2. クロージャ（Closures）
// 環境の変数をキャプチャできる匿名関数

#[allow(unused)]
fn closure_basics() {
    // 基本的なクロージャ
    let add_one = |x| x + 1;
    println!("add_one(5) = {}", add_one(5));
    
    // 型を明示的に指定
    let multiply: fn(i32, i32) -> i32 = |x, y| x * y;
    println!("multiply(3, 4) = {}", multiply(3, 4));
    
    // 複数行のクロージャ
    let complex_calc = |x: i32, y: i32| {
        let sum = x + y;
        let product = x * y;
        sum * product
    };
    println!("complex_calc(3, 4) = {}", complex_calc(3, 4));
}

// 3. クロージャのキャプチャ
#[allow(unused)]
fn closure_capture() {
    let multiplier = 10;
    
    // 環境変数をキャプチャ
    let multiply_by_ten = |x| x * multiplier;
    println!("multiply_by_ten(5) = {}", multiply_by_ten(5));
    
    // 可変キャプチャ
    let mut counter = 0;
    let mut increment_counter = || {
        counter += 1;
        counter
    };
    
    println!("Counter: {}", increment_counter());
    println!("Counter: {}", increment_counter());
    println!("Counter: {}", increment_counter());
    
    // 所有権のキャプチャ
    let data = vec![1, 2, 3, 4, 5];
    let process_data = move || {
        data.iter().map(|x| x * 2).collect::<Vec<i32>>()
    };
    
    println!("Processed data: {:?}", process_data());
    // println!("Original data: {:?}", data); // エラー: dataは移動済み
}

// 4. クロージャの型とトレイト
// Fn, FnMut, FnOnce の違いと使い分け

fn use_closure_fn<F>(f: F) 
where 
    F: Fn(i32) -> i32 
{
    println!("f(5) = {}", f(5));
    println!("f(10) = {}", f(10)); // 複数回呼び出し可能
}

fn use_closure_fn_mut<F>(mut f: F) 
where 
    F: FnMut(i32) -> i32 
{
    println!("f(5) = {}", f(5));
    println!("f(10) = {}", f(10));
}

fn use_closure_fn_once<F>(f: F) 
where 
    F: FnOnce(i32) -> i32 
{
    println!("f(5) = {}", f(5));
    // println!("f(10) = {}", f(10)); // エラー: 一度しか呼び出せない
}

#[allow(unused)]
fn closure_traits() {
    // Fn: 不変借用でキャプチャ
    let multiplier = 2;
    let double = |x| x * multiplier;
    use_closure_fn(double);
    
    // FnMut: 可変借用でキャプチャ
    let mut counter = 0;
    let count_and_add = |x| {
        counter += 1;
        x + counter
    };
    use_closure_fn_mut(count_and_add);
    
    // FnOnce: 値をキャプチャ
    let data = vec![1, 2, 3];
    let consume_data = |x| {
        println!("Data: {:?}", data);
        x
    };
    use_closure_fn_once(consume_data);
}

// 5. イテレータ（Iterators）
// 遅延評価による効率的なデータ処理

#[allow(unused)]
fn iterator_basics() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // イテレータの作成
    let iter = numbers.iter();
    
    // for文で使用
    for num in iter {
        println!("Number: {}", num);
    }
    
    // チェーンメソッド
    let result: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .filter(|&x| x > 4)
        .collect();
    
    println!("Result: {:?}", result);
}

// 6. カスタムイテレータの作成
struct Counter {
    current: i32,
    max: i32,
}

impl Counter {
    fn new(max: i32) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

#[allow(unused)]
fn custom_iterator() {
    let counter = Counter::new(5);
    
    for num in counter {
        println!("Count: {}", num);
    }
    
    // イテレータメソッドの使用
    let squares: Vec<i32> = Counter::new(10)
        .map(|x| x * x)
        .filter(|&x| x % 2 == 0)
        .collect();
    
    println!("Even squares: {:?}", squares);
}

// 7. 高度なイテレータパターン
#[allow(unused)]
fn advanced_iterator_patterns() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // enumerate: インデックス付きイテレータ
    let indexed: Vec<(usize, i32)> = numbers.iter().enumerate().map(|(i, &x)| (i, x)).collect();
    println!("Indexed: {:?}", indexed);
    
    // zip: 複数のイテレータを組み合わせ
    let letters = vec!['a', 'b', 'c', 'd', 'e'];
    let pairs: Vec<(i32, char)> = numbers.iter().zip(letters.iter()).map(|(&n, &c)| (n, c)).collect();
    println!("Pairs: {:?}", pairs);
    
    // take: 指定した数だけ取得
    let first_three: Vec<i32> = numbers.iter().take(3).cloned().collect();
    println!("First three: {:?}", first_three);
    
    // skip: 指定した数をスキップ
    let skip_three: Vec<i32> = numbers.iter().skip(3).cloned().collect();
    println!("Skip three: {:?}", skip_three);
    
    // cycle: 無限に繰り返し
    let cycled: Vec<i32> = numbers.iter().cycle().take(15).cloned().collect();
    println!("Cycled: {:?}", cycled);
}

// 8. 畳み込み演算（Fold operations）
#[allow(unused)]
fn fold_operations() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // fold: 汎用的な畳み込み
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum using fold: {}", sum);
    
    // reduce: 最初の要素を初期値として使用
    let product = numbers.iter().cloned().reduce(|acc, x| acc * x);
    println!("Product using reduce: {:?}", product);
    
    // scan: 中間結果も保持
    let running_sum: Vec<i32> = numbers.iter().scan(0, |state, &x| {
        *state += x;
        Some(*state)
    }).collect();
    println!("Running sum: {:?}", running_sum);
    
    // 複雑な畳み込み例
    let text = "Hello World Programming";
    let word_stats = text.split_whitespace().fold(
        (0, 0), // (単語数, 文字数)
        |(word_count, char_count), word| {
            (word_count + 1, char_count + word.len())
        }
    );
    println!("Words: {}, Characters: {}", word_stats.0, word_stats.1);
}

// 9. 関数型プログラミングでのエラーハンドリング
#[allow(unused)]
fn functional_error_handling() {
    let strings = vec!["42", "hello", "123", "world", "456"];
    
    // filter_map: 成功したもののみを処理
    let numbers: Vec<i32> = strings
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("Parsed numbers: {:?}", numbers);
    
    // partition: 成功と失敗を分離
    let (numbers, errors): (Vec<Result<i32, _>>, Vec<Result<i32, _>>) = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .partition(|r| r.is_ok());
    
    let valid_numbers: Vec<i32> = numbers.into_iter().map(|r| r.unwrap()).collect();
    println!("Valid numbers: {:?}", valid_numbers);
    
    // collect()でResult<Vec<_>, _>を作成
    let all_or_none: Result<Vec<i32>, _> = vec!["1", "2", "3"]
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    
    match all_or_none {
        Ok(nums) => println!("All parsed: {:?}", nums),
        Err(e) => println!("Parse error: {}", e),
    }
}

// 10. 高階関数とクロージャの組み合わせ
fn create_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn create_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

#[allow(unused)]
fn higher_order_closures() {
    let add_five = create_adder(5);
    let multiply_by_three = create_multiplier(3);
    
    println!("add_five(10) = {}", add_five(10));
    println!("multiply_by_three(7) = {}", multiply_by_three(7));
    
    // 関数の組み合わせ
    let numbers = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = numbers
        .iter()
        .map(|&x| add_five(x))
        .map(|x| multiply_by_three(x))
        .collect();
    
    println!("Combined operations: {:?}", result);
}

// 11. 実践的な例：データ処理パイプライン
#[derive(Debug)]
struct Sale {
    product: String,
    amount: f64,
    quantity: i32,
}

#[allow(unused)]
fn data_processing_pipeline() {
    let sales = vec![
        Sale { product: "Laptop".to_string(), amount: 1000.0, quantity: 2 },
        Sale { product: "Mouse".to_string(), amount: 25.0, quantity: 10 },
        Sale { product: "Keyboard".to_string(), amount: 75.0, quantity: 5 },
        Sale { product: "Monitor".to_string(), amount: 300.0, quantity: 3 },
        Sale { product: "Laptop".to_string(), amount: 1000.0, quantity: 1 },
    ];
    
    // 総売上を計算
    let total_revenue: f64 = sales
        .iter()
        .map(|sale| sale.amount * sale.quantity as f64)
        .sum();
    
    println!("Total revenue: ${:.2}", total_revenue);
    
    // 高額商品の分析
    let high_value_sales: Vec<&Sale> = sales
        .iter()
        .filter(|sale| sale.amount > 50.0)
        .collect();
    
    println!("High value sales: {:?}", high_value_sales);
    
    // 商品別の売上集計
    use std::collections::HashMap;
    let mut product_totals: HashMap<String, f64> = HashMap::new();
    
    sales.iter().for_each(|sale| {
        let total = product_totals.entry(sale.product.clone()).or_insert(0.0);
        *total += sale.amount * sale.quantity as f64;
    });
    
    println!("Product totals: {:?}", product_totals);
}

// メイン関数
fn main() {
    println!("=== Rust 関数型プログラミング 中級編 ===\n");
    
    println!("1. 高階関数の例:");
    println!("apply_twice(square, 3) = {}", apply_twice(square, 3));
    println!("apply_twice(increment, 5) = {}", apply_twice(increment, 5));
    
    let composed = compose(square, increment);
    println!("compose(square, increment)(4) = {}", composed(4));
    
    println!("\n2. クロージャの基本:");
    closure_basics();
    
    println!("\n3. クロージャのキャプチャ:");
    closure_capture();
    
    println!("\n4. クロージャの型とトレイト:");
    closure_traits();
    
    println!("\n5. イテレータの基本:");
    iterator_basics();
    
    println!("\n6. カスタムイテレータ:");
    custom_iterator();
    
    println!("\n7. 高度なイテレータパターン:");
    advanced_iterator_patterns();
    
    println!("\n8. 畳み込み演算:");
    fold_operations();
    
    println!("\n9. 関数型エラーハンドリング:");
    functional_error_handling();
    
    println!("\n10. 高階関数とクロージャの組み合わせ:");
    higher_order_closures();
    
    println!("\n11. データ処理パイプライン:");
    data_processing_pipeline();
}

// 練習問題とテスト
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_higher_order_functions() {
        assert_eq!(apply_twice(square, 3), 81);
        assert_eq!(apply_twice(increment, 5), 7);
        
        let composed = compose(square, increment);
        assert_eq!(composed(4), 25); // (4+1)^2 = 25
    }
    
    #[test]
    fn test_closures() {
        let multiplier = 5;
        let multiply_by_five = |x| x * multiplier;
        assert_eq!(multiply_by_five(3), 15);
        
        let add_ten = create_adder(10);
        assert_eq!(add_ten(5), 15);
    }
    
    #[test]
    fn test_custom_iterator() {
        let counter = Counter::new(3);
        let values: Vec<i32> = counter.collect();
        assert_eq!(values, vec![0, 1, 2]);
    }
    
    #[test]
    fn test_iterator_methods() {
        let numbers = vec![1, 2, 3, 4, 5];
        
        let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
        
        let sum: i32 = numbers.iter().sum();
        assert_eq!(sum, 15);
        
        let product: i32 = numbers.iter().product();
        assert_eq!(product, 120);
    }
    
    #[test]
    fn test_fold_operations() {
        let numbers = vec![1, 2, 3, 4, 5];
        
        let sum = numbers.iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15);
        
        let product = numbers.iter().fold(1, |acc, x| acc * x);
        assert_eq!(product, 120);
    }
}