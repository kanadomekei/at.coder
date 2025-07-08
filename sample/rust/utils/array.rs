
// 配列の全要素の合計を計算する
pub fn sum_vec(arr: &[i64]) -> i64 {
    arr.iter().sum()
}

// 配列の最大値を取得する
pub fn max_vec(arr: &[i64]) -> i64 {
    *arr.iter().max().unwrap()
}

// 配列の最小値を取得する
pub fn min_vec(arr: &[i64]) -> i64 {
    *arr.iter().min().unwrap()
}

// 配列を逆順にする（in-place）
pub fn reverse_vec<T>(arr: &mut [T]) {
    arr.reverse()
}

// 配列を昇順にソートする（in-place）
pub fn sort_vec<T: Ord>(arr: &mut [T]) {
    arr.sort();
}

// 配列を降順にソートする（in-place）
pub fn sort_vec_desc<T: Ord>(arr: &mut [T]) {
    arr.sort_by(|a, b| b.cmp(a));
}

// 二分探索で指定した値の位置を見つける（ソート済み配列で使用）
// 見つからない場合はNoneを返す
pub fn binary_search(arr: &[i64], target: i64) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

// target以上の最初の位置を見つける（ソート済み配列で使用）
// C++のlower_boundと同等
pub fn lower_bound(arr: &[i64], target: i64) -> usize {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

// targetより大きい最初の位置を見つける（ソート済み配列で使用）
// C++のupper_boundと同等
pub fn upper_bound(arr: &[i64], target: i64) -> usize {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] <= target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

// 配列内の重複する要素を見つける
// 戻り値：重複する要素のソート済みリスト
pub fn find_duplicates(arr: &[i64]) -> Vec<i64> {
    use std::collections::HashMap;
    let mut count = HashMap::new();
    let mut duplicates = Vec::new();
    
    for &x in arr {
        *count.entry(x).or_insert(0) += 1;
    }
    
    for (&key, &val) in &count {
        if val > 1 {
            duplicates.push(key);
        }
    }
    
    duplicates.sort();
    duplicates
}

// 配列内の重複を除いた要素を順番を保ったまま取得する
pub fn unique_elements(arr: &[i64]) -> Vec<i64> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    
    for &x in arr {
        if seen.insert(x) {
            result.push(x);
        }
    }
    
    result
}

// 配列から重複要素を除去する（in-place、ソート後に重複削除）
pub fn remove_duplicates(arr: &mut Vec<i64>) {
    arr.sort();
    arr.dedup();
}

// 配列を左にk個分回転させる（in-place）
// 例：[1,2,3,4,5]を2個左回転 → [3,4,5,1,2]
pub fn rotate_left<T: Clone>(arr: &mut [T], k: usize) {
    if arr.is_empty() || k == 0 {
        return;
    }
    
    let n = arr.len();
    let k = k % n;
    
    if k == 0 {
        return;
    }
    
    let mut temp = arr[0..k].to_vec();
    for i in 0..n - k {
        arr[i] = arr[i + k].clone();
    }
    for i in 0..k {
        arr[n - k + i] = temp[i].clone();
    }
}

// 配列を右にk個分回転させる（in-place）
// 例：[1,2,3,4,5]を2個右回転 → [4,5,1,2,3]
pub fn rotate_right<T: Clone>(arr: &mut [T], k: usize) {
    if arr.is_empty() || k == 0 {
        return;
    }
    
    let n = arr.len();
    let k = k % n;
    
    if k == 0 {
        return;
    }
    
    rotate_left(arr, n - k);
}

// 配列を次の順列に変更する（辞書順）
// 次の順列が存在する場合はtrueを返す
pub fn next_permutation<T: Ord>(arr: &mut [T]) -> bool {
    let n = arr.len();
    if n <= 1 {
        return false;
    }
    
    let mut i = n - 2;
    while i > 0 && arr[i] >= arr[i + 1] {
        i -= 1;
    }
    
    if i == 0 && arr[i] >= arr[i + 1] {
        return false;
    }
    
    let mut j = n - 1;
    while arr[j] <= arr[i] {
        j -= 1;
    }
    
    arr.swap(i, j);
    arr[i + 1..].reverse();
    true
}

// 配列を前の順列に変更する（辞書順）
// 前の順列が存在する場合はtrueを返す
pub fn prev_permutation<T: Ord>(arr: &mut [T]) -> bool {
    let n = arr.len();
    if n <= 1 {
        return false;
    }
    
    let mut i = n - 2;
    while i > 0 && arr[i] <= arr[i + 1] {
        i -= 1;
    }
    
    if i == 0 && arr[i] <= arr[i + 1] {
        return false;
    }
    
    let mut j = n - 1;
    while arr[j] >= arr[i] {
        j -= 1;
    }
    
    arr.swap(i, j);
    arr[i + 1..].reverse();
    true
}

// 配列の転倒数を計算する（i < j かつ arr[i] > arr[j] となるペアの個数）
// 時間計算量：O(n²)
pub fn inversion_count(arr: &[i64]) -> i64 {
    let n = arr.len();
    let mut count = 0;
    
    for i in 0..n {
        for j in i + 1..n {
            if arr[i] > arr[j] {
                count += 1;
            }
        }
    }
    
    count
}

// 最長増加部分列（LIS）の長さを計算する
// 時間計算量：O(n²)
pub fn longest_increasing_subsequence(arr: &[i64]) -> i64 {
    if arr.is_empty() {
        return 0;
    }
    
    let mut dp = vec![1; arr.len()];
    
    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] < arr[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    
    *dp.iter().max().unwrap()
}

// 最大部分配列和を計算する（Kadaneのアルゴリズム）
// 連続する部分配列の和の最大値を求める
pub fn kadane_algorithm(arr: &[i64]) -> i64 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    
    for &x in arr.iter().skip(1) {
        current_sum = current_sum.max(x);
        current_sum += x;
        max_sum = max_sum.max(current_sum);
    }
    
    max_sum
}

// 2つの数の和がtargetになるペアを見つける
// 戻り値：(インデックス1, インデックス2)
pub fn two_sum(arr: &[i64], target: i64) -> Option<(usize, usize)> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    
    for (i, &x) in arr.iter().enumerate() {
        let complement = target - x;
        if let Some(&j) = map.get(&complement) {
            return Some((j, i));
        }
        map.insert(x, i);
    }
    
    None
}

// 3つの数の和がtargetになるトリプルを全て見つける
// 重複するトリプルは除外される
pub fn three_sum(arr: &[i64], target: i64) -> Vec<Vec<i64>> {
    let mut arr = arr.to_vec();
    arr.sort();
    let mut result = Vec::new();
    let n = arr.len();
    
    for i in 0..n {
        if i > 0 && arr[i] == arr[i - 1] {
            continue;
        }
        
        let mut left = i + 1;
        let mut right = n - 1;
        
        while left < right {
            let sum = arr[i] + arr[left] + arr[right];
            if sum == target {
                result.push(vec![arr[i], arr[left], arr[right]]);
                
                while left < right && arr[left] == arr[left + 1] {
                    left += 1;
                }
                while left < right && arr[right] == arr[right - 1] {
                    right -= 1;
                }
                
                left += 1;
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    
    result
}

// 2つのソート済み配列をマージして1つのソート済み配列にする
pub fn merge_sorted_arrays(arr1: &[i64], arr2: &[i64]) -> Vec<i64> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    
    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }
    
    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }
    
    result
}

// 配列をpivot以下とpivotより大きい要素に分割する
// 戻り値：pivot以下の要素の個数
pub fn partition_array(arr: &mut [i64], pivot: i64) -> usize {
    let mut i = 0;
    
    for j in 0..arr.len() {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    i
}

// k番目に小さい要素を見つける（0-indexed）
// クイックセレクトアルゴリズム、平均時間計算量：O(n)
pub fn quick_select(arr: &mut [i64], k: usize) -> i64 {
    if arr.len() == 1 {
        return arr[0];
    }
    
    let pivot = arr[arr.len() / 2];
    let pivot_index = partition_array(arr, pivot);
    
    if k < pivot_index {
        quick_select(&mut arr[0..pivot_index], k)
    } else if k > pivot_index {
        quick_select(&mut arr[pivot_index..], k - pivot_index)
    } else {
        pivot
    }
} 