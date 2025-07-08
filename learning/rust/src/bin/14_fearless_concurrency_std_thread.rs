use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// # 1. 並行処理 (Concurrency) の例
///
/// `std::thread` を使って、メインスレッドとは別のスレッドを生成し、
/// そこで独立したタスクを実行させる例です。
fn main() {
    println!("--- 1. 所有権をムーブする単純な並行処理 ---");

    let v = vec![1, 2, 3, 4, 5];

    // `thread::spawn`で新しいスレッドを開始します。
    // `move`クロージャは、使用する変数の所有権を新しいスレッドに「移動」させることを指示します。
    // ここでは、ベクトル`v`の所有権がメインスレッドから新しいスレッドへ移ります。
    let handle = thread::spawn(move || {
        println!("別スレッド: ベクトル `v` の所有権を受け取りました: {:?}", v);
        // ここで何らかの重い計算をすると仮定
        let sum: i32 = v.iter().sum();
        thread::sleep(Duration::from_millis(1));
        println!("別スread: 計算が完了しました。");
        sum // スレッドの最後に評価された値が、このスレッドの返り値になります
    });

    // `v`の所有権は既に新しいスレッドに移動しているため、メインスレッドからはアクセスできません。
    // 以下の行のコメントを外すと、コンパイルエラーになります。
    // println!("メインスレッド: `v` はもう使えません: {:?}", v);
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ error[E0382]: borrow of moved value: `v`
    // これが、Rustのコンパイラが「データ競合」を未然に防いでくれる仕組みです。

    println!("メインスread: 別スレッドの処理が終わるのを待ちます...");
    // `handle.join()` は、生成したスレッドの処理が終わるのを待ち、その結果を受け取ります。
    let sum = handle.join().unwrap();

    println!(
        "メインスレッド: 別スreadから結果を受け取りました。合計は {} です。",
        sum
    );

    println!("\n--- 2. Arc<Mutex<T>> でデータを共有する並行処理 ---");

    // 複数のスレッドからデータを「共有」して変更したい場合は、`Arc<Mutex<T>>` を使います。
    // Arc:   スレッドセーフな参照カウンタ (Atomically Reference Counted)
    // Mutex: 상호 배제 (Mutual Exclusion) のためのロック

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        // `Arc::clone`でカウンタへの参照をクローンします。コストは非常に低いです。
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // `lock()`でロックを取得し、他のスレッドが同時にアクセスできないようにします。
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!(
                "スレッド {}: カウンタをインクリメントしました。現在の値: {}",
                i, *num
            );
            // `num` (MutexGuard) がこのスコープを抜けるときに、自動的にロックが解放されます。
        });
        handles.push(handle);
    }

    // すべてのスレッドが終了するのを待つ
    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "\n最終結果: 10個のスレッドがすべて完了しました。カウンタの値: {}",
        *counter.lock().unwrap()
    );
}
