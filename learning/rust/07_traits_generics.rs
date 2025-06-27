use std::fmt::Display;

fn main() {
    println!("=== Rustのトレイトとジェネリクス ===");
    
    // トレイトの基本
    let article = NewsArticle {
        headline: String::from("重要なニュース"),
        location: String::from("東京"),
        author: String::from("記者A"),
        content: String::from("今日の重要なニュースです..."),
    };
    
    println!("記事の要約: {}", article.summarize());
    println!("デフォルト実装: {}", article.summarize_default());
    
    // ジェネリクス
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大の数値: {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大の文字: {}", result);
    
    // ジェネリック構造体
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = MixedPoint { x: 5, y: 4.0 };
    
    println!("整数ポイント: x={}, y={}", integer.x, integer.y);
    println!("浮動小数点ポイント: x={}, y={}", float.x, float.y);
    println!("混合ポイント: x={}, y={}", mixed.x, mixed.y);
    
    // トレイト境界
    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("今日はいい天気ですね！"),
        reply: false,
        retweet: false,
    };
    
    notify(&tweet);
    notify_display(&tweet);
}

// トレイト定義
trait Summary {
    fn summarize(&self) -> String;
    
    // デフォルト実装
    fn summarize_default(&self) -> String {
        String::from("(続きを読む...)")
    }
}

// 構造体定義
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// トレイト実装
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.username, self.content)
    }
}

// ジェネリック関数
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// ジェネリック構造体
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// トレイト境界を持つ関数
fn notify(item: &impl Summary) {
    println!("速報: {}", item.summarize());
}

fn notify_display<T: Summary + Display>(item: &T) {
    println!("表示: {}", item);
    println!("要約: {}", item.summarize());
}