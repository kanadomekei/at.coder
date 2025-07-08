// `rayon::prelude::*` をインポートすることで、`.par_iter()`などの便利なメソッドが使えるようになります。
use rayon::prelude::*;
use std::time::Instant;

/// # 2. 並列処理 (Parallelism) の例
///
/// `rayon`クレートを使って、巨大なデータセットの処理を簡単に並列化する例です。
/// CPUのマルチコア性能を最大限に引き出し、処理を高速化することを目的とします。
fn main() {
    println!("=== 逐次処理 vs 並列処理 比較デモ ===");
    println!("CPUコア数: {}", rayon::current_num_threads());

    // 1. 基本的な変換処理の比較
    basic_transformation_comparison();

    // 2. 合計計算の比較
    sum_calculation_comparison();

    // 3. 複雑な計算の比較
    complex_calculation_comparison();
}

/// 基本的な変換処理（各要素を2倍にする）の逐次処理と並列処理の比較
fn basic_transformation_comparison() {
    println!("\n--- 1. 基本的な変換処理（各要素を2倍）の比較 ---");
    let size = 50_000_000;

    // 逐次処理
    let mut numbers_seq: Vec<i64> = (0..size).collect();
    let start_seq = Instant::now();
    numbers_seq.iter_mut().for_each(|n| *n *= 2);
    let duration_seq = start_seq.elapsed();

    // 並列処理
    let mut numbers_par: Vec<i64> = (0..size).collect();
    let start_par = Instant::now();
    numbers_par.par_iter_mut().for_each(|n| *n *= 2);
    let duration_par = start_par.elapsed();

    println!("  要素数: {}", size);
    println!("  逐次処理時間: {:?}", duration_seq);
    println!("  並列処理時間: {:?}", duration_par);

    if duration_par < duration_seq {
        let improvement = duration_seq.as_micros() as f64 / duration_par.as_micros() as f64;
        println!("  🚀 並列処理は逐次処理より {:.2} 倍高速！", improvement);
    } else {
        println!("  ⚠️ 並列処理は逐次処理より高速ではありませんでした");
    }

    // 結果の検証
    let first_10_seq: Vec<i64> = numbers_seq.iter().take(10).copied().collect();
    let first_10_par: Vec<i64> = numbers_par.iter().take(10).copied().collect();
    println!("  結果確認 - 逐次処理の最初の10個: {:?}", first_10_seq);
    println!("  結果確認 - 並列処理の最初の10個: {:?}", first_10_par);
    println!("  結果の一致: {}", first_10_seq == first_10_par);
}

/// 合計計算の逐次処理と並列処理の比較
fn sum_calculation_comparison() {
    println!("\n--- 2. 合計計算の比較 ---");
    let size = 100_000_000;
    let numbers: Vec<i64> = (1..=size).collect();

    // 逐次処理
    let start_seq = Instant::now();
    let sum_seq: i64 = numbers.iter().sum();
    let duration_seq = start_seq.elapsed();

    // 並列処理
    let start_par = Instant::now();
    let sum_par: i64 = numbers.par_iter().sum();
    let duration_par = start_par.elapsed();

    println!("  要素数: {}", size);
    println!("  逐次処理時間: {:?}", duration_seq);
    println!("  並列処理時間: {:?}", duration_par);

    if duration_par < duration_seq {
        let improvement = duration_seq.as_micros() as f64 / duration_par.as_micros() as f64;
        println!("  🚀 並列処理は逐次処理より {:.2} 倍高速！", improvement);
    } else {
        println!("  ⚠️ 並列処理は逐次処理より高速ではありませんでした");
    }

    println!("  結果の一致: {}", sum_seq == sum_par);
    println!("  合計値: {}", sum_seq);
}

/// 複雑な計算（平方根の計算）の逐次処理と並列処理の比較
fn complex_calculation_comparison() {
    println!("\n--- 3. 複雑な計算（平方根）の比較 ---");
    let size = 20_000_000;
    let numbers: Vec<f64> = (1..=size).map(|x| x as f64).collect();

    // 逐次処理
    let start_seq = Instant::now();
    let sqrt_seq: Vec<f64> = numbers.iter().map(|x| x.sqrt()).collect();
    let duration_seq = start_seq.elapsed();

    // 並列処理
    let start_par = Instant::now();
    let sqrt_par: Vec<f64> = numbers.par_iter().map(|x| x.sqrt()).collect();
    let duration_par = start_par.elapsed();

    println!("  要素数: {}", size);
    println!("  逐次処理時間: {:?}", duration_seq);
    println!("  並列処理時間: {:?}", duration_par);

    if duration_par < duration_seq {
        let improvement = duration_seq.as_micros() as f64 / duration_par.as_micros() as f64;
        println!("  🚀 並列処理は逐次処理より {:.2} 倍高速！", improvement);
    } else {
        println!("  ⚠️ 並列処理は逐次処理より高速ではありませんでした");
    }

    // 結果の検証（最初の5個の要素）
    let first_5_seq: Vec<f64> = sqrt_seq.iter().take(5).copied().collect();
    let first_5_par: Vec<f64> = sqrt_par.iter().take(5).copied().collect();
    println!("  結果確認 - 逐次処理の最初の5個: {:?}", first_5_seq);
    println!("  結果確認 - 並列処理の最初の5個: {:?}", first_5_par);

    println!("\n=== まとめ ===");
    println!("💡 並列処理のメリット:");
    println!("   - CPUのマルチコアを活用して処理を高速化");
    println!("   - .iter() を .par_iter() に変えるだけで簡単に並列化");
    println!("   - 大きなデータセットや計算負荷の高い処理で効果的");
    println!("⚠️ 注意点:");
    println!("   - 小さなデータセットでは並列化のオーバーヘッドで遅くなる場合も");
    println!("   - スレッド間の同期コストが発生する");
}
