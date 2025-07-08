use rand::prelude::*;
use rayon::join;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    println!("=== Rayon 高度な並列処理 - 逐次処理との比較 ===");
    println!("CPUコア数: {}", rayon::current_num_threads());

    println!("\n--- 1. 並列ソート vs 逐次ソート ---");
    parallel_sort_demo();

    println!("\n--- 2. 並列検索 vs 逐次検索 ---");
    parallel_find_demo();

    println!("\n--- 3. 独立したタスクの並列実行 vs 逐次実行 ---");
    join_demo();

    println!("\n--- 4. スコープ付きタスク（安全な並列処理） ---");
    scope_demo();
}

/// 巨大なベクターを並列でソートする
fn parallel_sort_demo() {
    let mut rng = rand::thread_rng();
    let size = 10_000_000;
    let mut large_vec: Vec<u32> = (0..size).collect();
    large_vec.shuffle(&mut rng);

    println!("  要素数: {}", size);
    println!("  🔀 ランダムにシャッフルされたデータをソート");

    // 逐次ソート
    let mut vec_seq = large_vec.clone();
    let start_seq = Instant::now();
    vec_seq.sort();
    let duration_seq = start_seq.elapsed();
    println!("  逐次ソート時間: {:?}", duration_seq);

    // 並列ソート
    let mut vec_par = large_vec.clone();
    let start_par = Instant::now();
    vec_par.par_sort(); // .sort() を .par_sort() に変えるだけ
    let duration_par = start_par.elapsed();
    println!("  並列ソート時間: {:?}", duration_par);

    if duration_par < duration_seq {
        let improvement = duration_seq.as_micros() as f64 / duration_par.as_micros() as f64;
        println!(
            "  🚀 並列ソートは逐次ソートより {:.2} 倍高速！",
            improvement
        );
    } else {
        println!("  ⚠️ 並列ソートは逐次ソートより高速ではありませんでした");
    }

    // 結果の確認
    let first_10_seq: Vec<u32> = vec_seq.iter().take(10).copied().collect();
    let first_10_par: Vec<u32> = vec_par.iter().take(10).copied().collect();
    println!("  結果の一致: {}", first_10_seq == first_10_par);
    println!("  ソート結果の最初の10個: {:?}", first_10_seq);
}

/// 巨大なデータセットから条件に合う要素を並列で探す
fn parallel_find_demo() {
    let size = 50_000_000;
    let huge_vector: Vec<i64> = (1..=size).collect();
    let target = size - 1; // 最後の方にある値を探す

    println!("  要素数: {}", size);
    println!("  🔍 値 {} を探索", target);

    // 逐次検索
    let start_seq = Instant::now();
    let result_seq = huge_vector.iter().find(|&&x| x == target);
    let duration_seq = start_seq.elapsed();

    // 並列検索
    let start_par = Instant::now();
    let result_par = huge_vector.par_iter().find_any(|&&x| x == target);
    let duration_par = start_par.elapsed();

    println!("  逐次検索時間: {:?}", duration_seq);
    println!("  並列検索時間: {:?}", duration_par);

    match (result_seq, result_par) {
        (Some(found_seq), Some(_found_par)) => {
            println!("  ✅ 両方とも値 {} を発見", found_seq);
            if duration_par < duration_seq {
                let improvement = duration_seq.as_micros() as f64 / duration_par.as_micros() as f64;
                println!("  🚀 並列検索は逐次検索より {:.2} 倍高速！", improvement);
            } else {
                println!("  ⚠️ 並列検索は逐次検索より高速ではありませんでした");
            }
        }
        _ => println!("  ❌ 値が見つかりませんでした"),
    }
}

/// 互いに独立した2つの重い処理を並列で実行する
fn join_demo() {
    println!("  🔄 2つの独立したタスクを実行");

    // 逐次実行
    let start_seq = Instant::now();
    let sum_seq = (1..10_000_000).sum::<u64>();
    let product_seq = (1..=20).product::<u64>();
    let duration_seq = start_seq.elapsed();

    // 並列実行
    let start_par = Instant::now();
    let (sum_par, product_par) = join(
        || (1..10_000_000).sum::<u64>(),
        || (1..=20).product::<u64>(), // 20!
    );
    let duration_par = start_par.elapsed();

    println!("  逐次実行時間: {:?}", duration_seq);
    println!("  並列実行時間: {:?}", duration_par);

    println!("  合計計算の結果: {}", sum_par);
    println!("  階乗計算の結果: {}", product_par);
    println!(
        "  結果の一致: {}",
        sum_seq == sum_par && product_seq == product_par
    );

    if duration_par < duration_seq {
        let improvement = duration_seq.as_micros() as f64 / duration_par.as_micros() as f64;
        println!("  🚀 並列実行は逐次実行より {:.2} 倍高速！", improvement);
    } else {
        println!("  ⚠️ 並列実行は逐次実行より高速ではありませんでした");
    }
}

/// スタック上のデータを安全に借用して並列処理を行う
fn scope_demo() {
    println!("  🔒 スコープ付き並列処理（安全な並列配列操作）");
    let mut array = [0; 1000];

    println!("  スコープ処理前の配列（最初の10個）: {:?}", &array[..10]);

    // 逐次処理での配列初期化
    let mut array_seq = [0; 1000];
    let start_seq = Instant::now();
    array_seq.iter_mut().enumerate().for_each(|(i, x)| {
        *x = if i < 500 { 1 } else { 2 };
    });
    let duration_seq = start_seq.elapsed();

    // 並列処理での配列初期化
    let start_par = Instant::now();
    let (first_half, second_half) = array.split_at_mut(500);
    rayon::scope(|s| {
        s.spawn(move |_| {
            first_half.iter_mut().for_each(|x| *x = 1);
        });
        s.spawn(|_| {
            second_half.iter_mut().for_each(|x| *x = 2);
        });
    });
    let duration_par = start_par.elapsed();

    println!("  逐次処理時間: {:?}", duration_seq);
    println!("  並列処理時間: {:?}", duration_par);
    println!("  スコープ処理後の配列（最初の10個）: {:?}", &array[..10]);
    println!("  スコープ処理後の配列（最後の10個）: {:?}", &array[990..]);

    // 結果の確認
    let results_match = array_seq == array;
    println!("  結果の一致: {}", results_match);

    if duration_par < duration_seq {
        let improvement = duration_seq.as_micros() as f64 / duration_par.as_micros() as f64;
        println!("  🚀 並列処理は逐次処理より {:.2} 倍高速！", improvement);
    } else {
        println!("  ⚠️ 並列処理は逐次処理より高速ではありませんでした");
    }

    println!("\n=== 高度な並列処理まとめ ===");
    println!("💡 Rayonの高度な機能:");
    println!("   - par_sort(): 並列ソート");
    println!("   - find_any(): 並列検索（順序不問）");
    println!("   - join(): 2つのタスクを並列実行");
    println!("   - scope(): スタック上のデータを安全に並列処理");
    println!("⚠️ 注意点:");
    println!("   - 処理コストと並列化コストのバランスが重要");
    println!("   - データサイズが小さい場合は逐次処理の方が速い場合も");
}
