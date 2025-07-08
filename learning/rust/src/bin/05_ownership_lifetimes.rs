fn main() {
    // ==========================================
    // 所有権とライフタイム
    // ==========================================

    println!("=== Rustの所有権とライフタイム ===");

    // ------------------------------------------
    // 所有権の基本
    // ------------------------------------------

    println!("\n=== 所有権の基本 ===");

    // 所有権の移動（move）
    {
        let s1 = String::from("hello");
        let s2 = s1; // s1の所有権がs2に移動

        // println!("s1: {}", s1); // エラー：s1はもう使用できない
        println!("s2: {}", s2);
    }

    // コピー可能な型（Copy trait）
    {
        let x = 5;
        let y = x; // 整数はコピーされる

        println!("x: {}, y: {}", x, y); // 両方とも使用可能
    }

    // 関数と所有権
    {
        let s = String::from("hello");
        takes_ownership(s); // sの所有権が関数に移動
                            // println!("s: {}", s); // エラー：sはもう使用できない

        let x = 5;
        makes_copy(x); // 整数はコピーされる
        println!("x: {}", x); // xは引き続き使用可能
    }

    // 関数からの所有権の返却
    {
        let s1 = gives_ownership(); // 関数が所有権を返す
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2); // s2の所有権を受け取り、返す

        println!("s1: {}, s3: {}", s1, s3);
        // s2はもう使用できない
    }

    // ------------------------------------------
    // 参照と借用
    // ------------------------------------------

    println!("\n=== 参照と借用 ===");

    // 不変参照
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1); // 参照を渡す

        println!("'{}' の長さは {} です", s1, len); // s1は引き続き使用可能
    }

    // 可変参照
    {
        let mut s = String::from("hello");
        change(&mut s); // 可変参照を渡す

        println!("変更後: {}", s);
    }

    // 参照の規則
    {
        let mut s = String::from("hello");

        // 複数の不変参照は問題なし
        let r1 = &s;
        let r2 = &s;
        println!("r1: {}, r2: {}", r1, r2);

        // 不変参照が使われなくなった後なら可変参照は可能
        let r3 = &mut s;
        println!("r3: {}", r3);
    }

    // ------------------------------------------
    // ダングリングポインタの防止
    // ------------------------------------------

    println!("\n=== ダングリングポインタの防止 ===");

    // コンパイルエラーになる例（コメントアウト）
    // let reference_to_nothing = dangle();

    // 正しい方法
    let valid_string = no_dangle();
    println!("有効な文字列: {}", valid_string);

    // ------------------------------------------
    // スライス
    // ------------------------------------------

    println!("\n=== スライス ===");

    // 文字列スライス
    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
        let hello_world = &s[..]; // 全体

        println!("hello: {}, world: {}, 全体: {}", hello, world, hello_world);

        // 最初の単語を取得
        let word = first_word(&s);
        println!("最初の単語: {}", word);
    }

    // 配列スライス
    {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];

        println!("配列スライス: {:?}", slice);
        assert_eq!(slice, &[2, 3]);
    }

    // ------------------------------------------
    // ライフタイム
    // ------------------------------------------

    println!("\n=== ライフタイム ===");

    // ライフタイム注釈が必要な例
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(&string1, string2);
        println!("最も長い文字列: {}", result);
    }

    // ライフタイムの範囲
    {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(&string1, &string2);
            println!("内側スコープでの結果: {}", result);
        }
        // result の使用はここまで有効
        // println!("外側スコープでの結果: {}", result); // エラーになる場合がある
    }

    // ------------------------------------------
    // 構造体のライフタイム
    // ------------------------------------------

    println!("\n=== 構造体のライフタイム ===");

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("文が見つかりませんでした");

        let i = ImportantExcerpt {
            part: first_sentence,
        };

        println!("重要な抜粋: {}", i.part);

        // メソッドの使用
        println!("レベル: {}", i.level());
        println!(
            "アナウンス: {}",
            i.announce_and_return_part("注意してください")
        );
    }

    // ------------------------------------------
    // 静的ライフタイム
    // ------------------------------------------

    println!("\n=== 静的ライフタイム ===");

    let s: &'static str = "I have a static lifetime.";
    println!("静的ライフタイム: {}", s);

    // ------------------------------------------
    // 所有権パターンの実践例
    // ------------------------------------------

    println!("\n=== 所有権パターンの実践例 ===");

    // ベクターの操作
    {
        let mut v = vec![1, 2, 3, 4, 5];

        // 不変参照でのアクセス
        for item in &v {
            println!("値: {}", item);
        }

        // 可変参照での変更
        for item in &mut v {
            *item *= 2;
        }

        println!("変更後のベクター: {:?}", v);

        // 所有権を移動する反復
        for item in v {
            println!("所有権移動: {}", item);
        }
        // v はもう使用できない
    }

    // 文字列の分割と処理
    {
        let data = String::from("apple,banana,cherry");
        let fruits: Vec<&str> = data.split(',').collect();

        println!("果物リスト: {:?}", fruits);

        // クローンを使った所有権の回避
        let owned_fruits: Vec<String> = fruits.iter().map(|s| s.to_string()).collect();
        println!("所有権を持つ果物リスト: {:?}", owned_fruits);
    }

    // Rc（Reference Counted）の使用例
    use std::rc::Rc;
    {
        let data = Rc::new(String::from("共有データ"));
        let data1 = Rc::clone(&data);
        let data2 = Rc::clone(&data);

        println!("参照カウント: {}", Rc::strong_count(&data));
        println!("data: {}", data);
        println!("data1: {}", data1);
        println!("data2: {}", data2);
    }

    // RefCell を使った内部可変性
    use std::cell::RefCell;
    {
        let value = RefCell::new(5);

        // 借用チェックは実行時に行われる
        {
            let borrowed = value.borrow();
            println!("借用した値: {}", *borrowed);
        }

        {
            let mut borrowed_mut = value.borrow_mut();
            *borrowed_mut += 10;
        }

        println!("変更後の値: {}", value.borrow());
    }

    // 追加のデモ関数を呼び出し
    demo_longest_with_announcement();
}

// ------------------------------------------
// 関数定義
// ------------------------------------------

// 所有権を受け取る関数
fn takes_ownership(some_string: String) {
    println!("所有権を受け取りました: {}", some_string);
} // some_string はここでドロップされる

// コピーを受け取る関数
fn makes_copy(some_integer: i32) {
    println!("コピーを受け取りました: {}", some_integer);
} // some_integer はここでスコープを抜ける

// 所有権を与える関数
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

// 所有権を受け取り、返す関数
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// 参照を受け取る関数
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 可変参照を受け取る関数
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// ダングリングポインタを作ろうとする関数（コメントアウト）
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s はここでドロップされるため、参照は無効になる

// 正しい方法
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// 最初の単語を返す関数
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ライフタイム注釈付きの関数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ライフタイム注釈付きの構造体
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // ライフタイム省略規則により注釈不要
    fn level(&self) -> i32 {
        3
    }

    // 複数のライフタイムパラメータ
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意: {}", announcement);
        self.part
    }
}

// ジェネリック型、トレイト境界、ライフタイムを組み合わせた例
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("アナウンス: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// longest_with_an_announcement関数のデモ用追加
fn demo_longest_with_announcement() {
    println!("\n=== ライフタイム付き関数のデモ ===");
    let announcement = "新機能のお知らせ";
    let result = longest_with_an_announcement("hello", "world", announcement);
    println!("最長の文字列: {}", result);
}
