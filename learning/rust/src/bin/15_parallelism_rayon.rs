// `rayon::prelude::*` をインポートすることで、`.par_iter()`などの便利なメソッドが使えるようになります。
use rayon::prelude::*;
use std::time::Instant;

/// # 2. 並列処理 (Parallelism) の例
///
/// `rayon`クレートを使って、巨大なデータセットの処理を簡単に並列化する例です。
/// CPUのマルチコア性能を最大限に引き出し、処理を高速化することを目的とします。
fn main() {
    let mut numbers: Vec<i64> = (0..20_000_000).collect();

    // --- 逐次処理 (Sequential processing) ---
    println!("--- 1. 逐次処理を開始します ---");
    let start_seq = Instant::now();

    // 通常のイテレータを使って、各要素を2倍にする
    numbers.iter_mut().for_each(|n| *n *= 2);

    let duration_seq = start_seq.elapsed();
    println!("逐次処理にかかった時間: {:?}", duration_seq);
    // numbersの値をリセット
    numbers = (0..20_000_000).collect();


    // --- 並列処理 (Parallel processing) ---
    println!("\n--- 2. Rayonを使った並列処理を開始します ---");
    let start_par = Instant::now();

    // `.iter_mut()` を `.par_iter_mut()` に変えるだけ！
    // これだけで、Rayonが裏側で自動的にデータを分割し、
    // 複数のスレッドに処理を割り当て、CPUコアをフル活用してくれます。
    numbers.par_iter_mut().for_each(|n| *n *= 2);

    let duration_par = start_par.elapsed();
    println!("並列処理にかかった時間: {:?}", duration_par);


    // --- 結果の比較 ---
    println!("\n--- 結果 ---");
    if duration_par < duration_seq {
        let improvement = (duration_seq.as_micros() as f64) / (duration_par.as_micros() as f64);
        println!("並列処理は逐次処理より {:.2} 倍高速でした。", improvement);
    } else {
        println!("並列処理は逐次処理より高速ではありませんでした。");
    }

    // --- `map-reduce` パターンの並列処理 ---
    // Rayonは、`sum()`や`max()`などの集計処理も簡単に並列化できます。
    println!("\n--- 3. 並列での合計計算 (Map-Reduce) ---");
    let huge_vector: Vec<i64> = (1..=1_000_000_000).collect();

    let start_sum = Instant::now();

    // `.par_iter()` を使って、巨大なベクターの合計値を並列に計算する
    let sum: i64 = huge_vector.par_iter().sum();
    // Rayonが内部でデータを複数のチャンクに分割 -> 各チャンクの合計を並列に計算 ->
    // 最後に各チャンクの結果を合計する、という処理を自動で行います。

    let duration_sum = start_sum.elapsed();
    println!("1から10億までの合計: {}", sum);
    println!("並列での合計計算にかかった時間: {:?}", duration_sum);
} 