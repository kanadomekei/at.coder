// 基本的な二分探索（配列内の値を検索）
pub fn binary_search_basic<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    
    None
}

// lower_bound（target以上の最小要素のインデックス）
pub fn lower_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    left
}

// upper_bound（targetより大きい最小要素のインデックス）
pub fn upper_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] <= *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    left
}

// 条件を満たす最小値を見つける二分探索
pub fn binary_search_min<F>(mut left: i64, mut right: i64, pred: F) -> i64
where
    F: Fn(i64) -> bool,
{
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if pred(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    right
}

// 条件を満たす最大値を見つける二分探索
pub fn binary_search_max<F>(mut left: i64, mut right: i64, pred: F) -> i64
where
    F: Fn(i64) -> bool,
{
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if pred(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

// 浮動小数点数での二分探索
pub fn binary_search_float<F>(mut left: f64, mut right: f64, pred: F) -> f64
where
    F: Fn(f64) -> bool,
{
    const EPS: f64 = 1e-9;
    
    while right - left > EPS {
        let mid = left + (right - left) / 2.0;
        if pred(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    
    (left + right) / 2.0
}

// 三分探索（単峰関数の最小値を求める）
pub fn ternary_search_min<F>(mut left: f64, mut right: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    const EPS: f64 = 1e-9;
    
    while right - left > EPS {
        let m1 = left + (right - left) / 3.0;
        let m2 = right - (right - left) / 3.0;
        
        if f(m1) > f(m2) {
            left = m1;
        } else {
            right = m2;
        }
    }
    
    (left + right) / 2.0
}

// 三分探索（単峰関数の最大値を求める）
pub fn ternary_search_max<F>(mut left: f64, mut right: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    const EPS: f64 = 1e-9;
    
    while right - left > EPS {
        let m1 = left + (right - left) / 3.0;
        let m2 = right - (right - left) / 3.0;
        
        if f(m1) < f(m2) {
            left = m1;
        } else {
            right = m2;
        }
    }
    
    (left + right) / 2.0
}

// 三分探索（整数版、単峰関数の最小値位置を求める）
pub fn ternary_search_min_int<F>(mut left: i64, mut right: i64, f: F) -> i64
where
    F: Fn(i64) -> i64,
{
    while right - left > 2 {
        let m1 = left + (right - left) / 3;
        let m2 = right - (right - left) / 3;
        
        if f(m1) > f(m2) {
            left = m1;
        } else {
            right = m2;
        }
    }
    
    let mut min_val = f(left);
    let mut min_pos = left;
    
    for i in left..=right {
        let val = f(i);
        if val < min_val {
            min_val = val;
            min_pos = i;
        }
    }
    
    min_pos
}

// 黄金比探索（三分探索より効率的）
pub fn golden_section_search<F>(mut left: f64, mut right: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    const PHI: f64 = 1.618033988749;
    const EPS: f64 = 1e-9;
    
    let mut x1 = right - (right - left) / PHI;
    let mut x2 = left + (right - left) / PHI;
    let mut f1 = f(x1);
    let mut f2 = f(x2);
    
    while (right - left).abs() > EPS {
        if f1 > f2 {
            left = x1;
            x1 = x2;
            f1 = f2;
            x2 = left + (right - left) / PHI;
            f2 = f(x2);
        } else {
            right = x2;
            x2 = x1;
            f2 = f1;
            x1 = right - (right - left) / PHI;
            f1 = f(x1);
        }
    }
    
    (left + right) / 2.0
}

// 基本的な二分探索の使用例
pub fn search_examples() {
    // ソート済み配列での基本的な二分探索
    let arr = vec![1, 3, 5, 7, 9, 11, 13, 15];
    
    // 値を検索
    if let Some(index) = binary_search_basic(&arr, &7) {
        println!("Found 7 at index {}", index);
    } else {
        println!("7 not found");
    }
    
    // lower_bound（7以上の最小要素のインデックス）
    let lb = lower_bound(&arr, &7);
    println!("Lower bound of 7: index {}", lb);
    
    // upper_bound（7より大きい最小要素のインデックス）
    let ub = upper_bound(&arr, &7);
    println!("Upper bound of 7: index {}", ub);
    
    // 存在しない値の場合
    let lb_6 = lower_bound(&arr, &6);
    println!("Lower bound of 6: index {}", lb_6);
    
    // 条件を満たす最小値を見つける
    // 例: x^2 >= 50 となる最小のxを見つける
    let min_x = binary_search_min(1, 100, |x| x * x >= 50);
    println!("Min x where x^2 >= 50: {}", min_x);
    
    // 浮動小数点での二分探索
    // 例: sqrt(50)を求める
    let sqrt_50 = binary_search_float(0.0, 50.0, |x| x * x >= 50.0);
    println!("sqrt(50) ≈ {}", sqrt_50);
    
    // 三分探索の例
    // 例: f(x) = (x-5)^2 + 3 の最小値を求める
    let min_point = ternary_search_min(0.0, 10.0, |x| (x - 5.0).powi(2) + 3.0);
    println!("Minimum point of f(x) = (x-5)^2 + 3: x = {}", min_point);
}

// 二分探索で解ける典型問題のパターン
pub fn binary_search_patterns() {
    // パターン1: 最大値の最小化
    // 例: 配列を k 個のグループに分けたときの各グループの合計の最大値を最小化
    let arr = vec![1, 2, 3, 4, 5];
    let k = 3;
    
    let can_divide = |max_sum: i64| -> bool {
        let mut groups = 1;
        let mut current_sum = 0;
        
        for &val in &arr {
            if current_sum + val > max_sum {
                groups += 1;
                current_sum = val;
                if groups > k {
                    return false;
                }
            } else {
                current_sum += val;
            }
        }
        
        groups <= k
    };
    
    let total_sum: i64 = arr.iter().sum();
    let max_element = *arr.iter().max().unwrap() as i64;
    
    let result = binary_search_min(max_element, total_sum, can_divide);
    println!("Minimum possible maximum sum: {}", result);
    
    // パターン2: 最小値の最大化
    // 例: 長さnのロープをk個に切ったときの最短の長さを最大化
    let rope_length = 100.0;
    let k_cuts = 5;
    
    let can_cut = |min_length: f64| -> bool {
        let cuts = (rope_length / min_length).floor() as i32;
        cuts >= k_cuts
    };
    
    let max_min_length = binary_search_float(0.0, rope_length, |x| !can_cut(x));
    println!("Maximum minimum length: {}", max_min_length);
}