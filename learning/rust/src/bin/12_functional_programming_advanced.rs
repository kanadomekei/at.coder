// Rust 関数型プログラミング - 上級編
// 関数合成、モナド的パターン、実践的な応用

use std::collections::HashMap;

// 1. 関数合成（Function Composition）
// 複数の関数を組み合わせて新しい関数を作成

trait Composable<A, B> {
    fn compose<C, G>(self, g: G) -> impl Fn(A) -> C
    where
        G: Fn(B) -> C,
        Self: Fn(A) -> B + Sized;
}

impl<A, B, F> Composable<A, B> for F
where
    F: Fn(A) -> B,
{
    fn compose<C, G>(self, g: G) -> impl Fn(A) -> C
    where
        G: Fn(B) -> C,
        Self: Fn(A) -> B + Sized,
    {
        move |x| g(self(x))
    }
}

// 基本的な関数
fn add_one(x: i32) -> i32 {
    x + 1
}

fn multiply_by_two(x: i32) -> i32 {
    x * 2
}

fn to_string(x: i32) -> String {
    format!("Value: {}", x)
}

#[allow(unused)]
fn function_composition_examples() {
    // 基本的な関数合成
    let add_then_multiply = |x| multiply_by_two(add_one(x));
    println!("add_then_multiply(5) = {}", add_then_multiply(5));

    // トレイトを使った関数合成
    let composed = add_one.compose(multiply_by_two).compose(to_string);
    println!("composed(3) = {}", composed(3));

    // パイプライン演算子的な使用
    let pipeline = |x| {
        vec![1, 2, 3, 4, 5]
            .iter()
            .map(|&n| n + x)
            .filter(|&n| n > 3)
            .collect::<Vec<i32>>()
    };

    println!("pipeline(2) = {:?}", pipeline(2));
}

// 2. 高階関数の組み合わせ
fn curry<A, B, C>(f: fn(A, B) -> C) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    A: Copy + 'static,
    B: 'static,
    C: 'static,
{
    move |a| Box::new(move |b| f(a, b))
}

fn uncurry<A, B, C, F, G>(f: F) -> impl Fn(A, B) -> C
where
    F: Fn(A) -> G,
    G: Fn(B) -> C,
{
    move |a, b| f(a)(b)
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

#[allow(unused)]
fn currying_examples() {
    // カリー化
    let curried_add = curry(add);
    let add_five = curried_add(5);
    println!("add_five(3) = {}", add_five(3));

    // 部分適用
    let numbers = vec![1, 2, 3, 4, 5];
    let incremented: Vec<i32> = numbers.iter().map(|&x| add_five(x)).collect();
    println!("Incremented: {:?}", incremented);

    // アンカリー化
    let curried_multiply = curry(multiply);
    let normal_multiply = uncurry(curried_multiply);
    println!("normal_multiply(3, 4) = {}", normal_multiply(3, 4));
}

// 3. モナド的パターン（Monadic Patterns）
// Option型とResult型の高度な使用

#[allow(unused)]
fn monadic_option_patterns() {
    // Option のチェーン
    let process_chain = |x: i32| {
        Some(x)
            .filter(|&n| n > 0)
            .map(|n| n * 2)
            .filter(|&n| n < 20)
            .map(|n| format!("Result: {}", n))
    };

    println!("process_chain(5) = {:?}", process_chain(5));
    println!("process_chain(15) = {:?}", process_chain(15));
    println!("process_chain(-5) = {:?}", process_chain(-5));

    // flatten の使用
    let nested_options = vec![Some(1), None, Some(2), None, Some(3)];
    let flattened: Vec<i32> = nested_options.into_iter().flatten().collect();
    println!("Flattened: {:?}", flattened);

    // and_then を使った複雑な処理
    let divide_and_sqrt = |x: f64| {
        Some(x)
            .filter(|&n| n != 0.0)
            .map(|n| 100.0 / n)
            .filter(|&n| n >= 0.0)
            .map(|n| n.sqrt())
    };

    println!("divide_and_sqrt(25.0) = {:?}", divide_and_sqrt(25.0));
    println!("divide_and_sqrt(0.0) = {:?}", divide_and_sqrt(0.0));
}

// 4. Result型の高度な使用
fn safe_divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(x / y)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err("Square root of negative number".to_string())
    } else {
        Ok(x.sqrt())
    }
}

#[allow(unused)]
fn monadic_result_patterns() {
    // Result のチェーン
    let complex_calculation = |x: f64, y: f64| {
        safe_divide(x, y)
            .and_then(|result| safe_sqrt(result))
            .map(|result| format!("Final result: {:.2}", result))
    };

    println!(
        "complex_calculation(100.0, 4.0) = {:?}",
        complex_calculation(100.0, 4.0)
    );
    println!(
        "complex_calculation(100.0, 0.0) = {:?}",
        complex_calculation(100.0, 0.0)
    );
    println!(
        "complex_calculation(-100.0, 4.0) = {:?}",
        complex_calculation(-100.0, 4.0)
    );

    // 複数のResult を組み合わせ
    let multiple_operations = |values: Vec<f64>| {
        values
            .iter()
            .map(|&x| safe_sqrt(x))
            .collect::<Result<Vec<f64>, String>>()
    };

    println!(
        "multiple_operations([4.0, 9.0, 16.0]) = {:?}",
        multiple_operations(vec![4.0, 9.0, 16.0])
    );
    println!(
        "multiple_operations([4.0, -9.0, 16.0]) = {:?}",
        multiple_operations(vec![4.0, -9.0, 16.0])
    );
}

// 5. 独自のモナド的型の作成
#[derive(Debug, Clone)]
struct Maybe<T> {
    value: Option<T>,
}

impl<T> Maybe<T> {
    fn new(value: T) -> Self {
        Maybe { value: Some(value) }
    }

    fn none() -> Self {
        Maybe { value: None }
    }

    fn map<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> U,
    {
        match self.value {
            Some(val) => Maybe::new(f(val)),
            None => Maybe::none(),
        }
    }

    fn flat_map<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> Maybe<U>,
    {
        match self.value {
            Some(val) => f(val),
            None => Maybe::none(),
        }
    }

    fn filter<F>(self, predicate: F) -> Self
    where
        F: FnOnce(&T) -> bool,
    {
        match self.value {
            Some(ref val) if predicate(val) => self,
            _ => Maybe::none(),
        }
    }
}

#[allow(unused)]
fn custom_monad_example() {
    let result = Maybe::new(10)
        .filter(|&x| x > 5)
        .map(|x| x * 2)
        .flat_map(|x| if x > 15 { Maybe::new(x) } else { Maybe::none() })
        .map(|x| format!("Final: {}", x));

    println!("Custom monad result: {:?}", result);

    // チェーンの途中でNoneになる例
    let empty_result = Maybe::new(3).filter(|&x| x > 5).map(|x| x * 2);

    println!("Empty result: {:?}", empty_result);
}

// 6. 関数型プログラミングでのデザインパターン
trait Functor<T> {
    type Mapped<U>;
    fn fmap<U, F>(self, f: F) -> Self::Mapped<U>
    where
        F: FnMut(T) -> U;
}

impl<T> Functor<T> for Option<T> {
    type Mapped<U> = Option<U>;

    fn fmap<U, F>(self, mut f: F) -> Self::Mapped<U>
    where
        F: FnMut(T) -> U,
    {
        self.map(|x| f(x))
    }
}

impl<T> Functor<T> for Vec<T> {
    type Mapped<U> = Vec<U>;

    fn fmap<U, F>(self, mut f: F) -> Self::Mapped<U>
    where
        F: FnMut(T) -> U,
    {
        self.into_iter().map(|x| f(x)).collect()
    }
}

#[allow(unused)]
fn functor_pattern_example() {
    // Option での Functor 使用
    let option_result = Some(42).fmap(|x| x * 2);
    println!("Option functor: {:?}", option_result);

    // Vec での Functor 使用は制限があるため、通常のmapを使用
    let vec_result: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x * 2).collect();
    println!("Vec result: {:?}", vec_result);
}

// 7. 実践的な関数型プログラミング例
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug, Clone)]
struct Order {
    id: u32,
    user_id: u32,
    amount: f64,
    items: Vec<String>,
}

#[allow(unused)]
fn practical_functional_example() {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            age: 30,
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            age: 25,
        },
        User {
            id: 3,
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
            age: 35,
        },
    ];

    let orders = vec![
        Order {
            id: 1,
            user_id: 1,
            amount: 150.0,
            items: vec!["Laptop".to_string(), "Mouse".to_string()],
        },
        Order {
            id: 2,
            user_id: 2,
            amount: 75.0,
            items: vec!["Keyboard".to_string()],
        },
        Order {
            id: 3,
            user_id: 1,
            amount: 300.0,
            items: vec!["Monitor".to_string()],
        },
        Order {
            id: 4,
            user_id: 3,
            amount: 50.0,
            items: vec!["Cable".to_string()],
        },
    ];

    // 関数型的なデータ処理

    // 1. ユーザーごとの総注文額
    let user_totals: HashMap<u32, f64> = orders.iter().fold(HashMap::new(), |mut acc, order| {
        *acc.entry(order.user_id).or_insert(0.0) += order.amount;
        acc
    });

    println!("User totals: {:?}", user_totals);

    // 2. 高額注文者の情報
    let high_value_users: Vec<(User, f64)> = users
        .iter()
        .filter_map(|user| {
            user_totals
                .get(&user.id)
                .map(|&total| (user.clone(), total))
        })
        .filter(|(_, total)| *total > 100.0)
        .collect();

    println!("High value users: {:?}", high_value_users);

    // 3. 注文アイテムの分析
    let all_items: Vec<String> = orders
        .iter()
        .flat_map(|order| order.items.iter())
        .cloned()
        .collect();

    let item_counts: HashMap<String, usize> =
        all_items.iter().fold(HashMap::new(), |mut acc, item| {
            *acc.entry(item.clone()).or_insert(0) += 1;
            acc
        });

    println!("Item counts: {:?}", item_counts);

    // 4. 年齢別の平均注文額
    let age_groups: HashMap<String, Vec<f64>> = users
        .iter()
        .filter_map(|user| {
            user_totals.get(&user.id).map(|&total| {
                let age_group = if user.age < 30 { "Young" } else { "Mature" };
                (age_group.to_string(), total)
            })
        })
        .fold(HashMap::new(), |mut acc, (age_group, total)| {
            acc.entry(age_group).or_insert_with(Vec::new).push(total);
            acc
        });

    let age_averages: HashMap<String, f64> = age_groups
        .into_iter()
        .map(|(group, totals)| {
            let average = totals.iter().sum::<f64>() / totals.len() as f64;
            (group, average)
        })
        .collect();

    println!("Age group averages: {:?}", age_averages);
}

// 8. 並列処理と関数型プログラミング
use std::sync::mpsc;
use std::thread;

#[allow(unused)]
fn parallel_functional_processing() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let chunk_size = 3;

    let (sender, receiver) = mpsc::channel();

    // データをチャンクに分割して並列処理
    let chunks: Vec<Vec<i32>> = data
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    for chunk in chunks {
        let sender = sender.clone();
        thread::spawn(move || {
            let result: i32 = chunk.iter().map(|x| x * x).sum();
            sender.send(result).unwrap();
        });
    }

    drop(sender);

    let results: Vec<i32> = receiver.iter().collect();
    let total: i32 = results.iter().sum();

    println!("Parallel processing results: {:?}", results);
    println!("Total: {}", total);
}

// 9. 状態管理の関数型アプローチ
#[derive(Debug, Clone)]
struct AppState {
    counter: i32,
    messages: Vec<String>,
}

#[derive(Debug)]
enum Action {
    Increment,
    Decrement,
    AddMessage(String),
    Reset,
}

fn reducer(state: AppState, action: Action) -> AppState {
    match action {
        Action::Increment => AppState {
            counter: state.counter + 1,
            ..state
        },
        Action::Decrement => AppState {
            counter: state.counter - 1,
            ..state
        },
        Action::AddMessage(msg) => {
            let mut new_messages = state.messages.clone();
            new_messages.push(msg);
            AppState {
                messages: new_messages,
                ..state
            }
        }
        Action::Reset => AppState {
            counter: 0,
            messages: Vec::new(),
        },
    }
}

#[allow(unused)]
fn state_management_example() {
    let initial_state = AppState {
        counter: 0,
        messages: Vec::new(),
    };

    let actions = vec![
        Action::Increment,
        Action::Increment,
        Action::AddMessage("Hello".to_string()),
        Action::Decrement,
        Action::AddMessage("World".to_string()),
    ];

    let final_state = actions.into_iter().fold(initial_state, reducer);

    println!("Final state: {:?}", final_state);
}

// メイン関数
fn main() {
    println!("=== Rust 関数型プログラミング 上級編 ===\n");

    println!("1. 関数合成の例:");
    function_composition_examples();

    println!("\n2. カリー化と部分適用:");
    currying_examples();

    println!("\n3. モナド的パターン - Option:");
    monadic_option_patterns();

    println!("\n4. モナド的パターン - Result:");
    monadic_result_patterns();

    println!("\n5. 独自モナドの例:");
    custom_monad_example();

    println!("\n6. Functor パターンの例:");
    functor_pattern_example();

    println!("\n7. 実践的な関数型プログラミング:");
    practical_functional_example();

    println!("\n8. 並列処理と関数型プログラミング:");
    parallel_functional_processing();

    println!("\n9. 状態管理の関数型アプローチ:");
    state_management_example();
}

// 練習問題とテスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_composition() {
        let composed = add_one.compose(multiply_by_two);
        assert_eq!(composed(5), 12); // (5 + 1) * 2 = 12
    }

    #[test]
    fn test_currying() {
        let curried_add = curry(add);
        let add_ten = curried_add(10);
        assert_eq!(add_ten(5), 15);
    }

    #[test]
    fn test_maybe_monad() {
        let result = Maybe::new(10).filter(|&x| x > 5).map(|x| x * 2);

        assert!(matches!(result.value, Some(20)));

        let empty_result = Maybe::new(3).filter(|&x| x > 5).map(|x| x * 2);

        assert!(matches!(empty_result.value, None));
    }

    #[test]
    fn test_result_chaining() {
        let result = safe_divide(100.0, 4.0).and_then(|x| safe_sqrt(x));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);

        let error_result = safe_divide(100.0, 0.0).and_then(|x| safe_sqrt(x));

        assert!(error_result.is_err());
    }

    #[test]
    fn test_reducer() {
        let initial_state = AppState {
            counter: 0,
            messages: Vec::new(),
        };

        let state = reducer(initial_state, Action::Increment);
        assert_eq!(state.counter, 1);

        let state = reducer(state, Action::AddMessage("test".to_string()));
        assert_eq!(state.messages.len(), 1);
        assert_eq!(state.messages[0], "test");
    }
}
