use std::collections::HashMap;

/// スライスから借用のイテレータを生成します。
pub fn create_iter_borrowed<T>(data: &[T]) -> std::slice::Iter<T> {
    data.iter()
}

/// `Vec`から所有権を持つイテレータを生成します。
pub fn create_iter_owned<T>(data: Vec<T>) -> std::vec::IntoIter<T> {
    data.into_iter()
}

/// スライスから可変借用のイテレータを生成します。
pub fn create_iter_mut<T>(data: &mut [T]) -> std::slice::IterMut<T> {
    data.iter_mut()
}

/// 範囲イテレータ（終端を含まない）を生成します。
pub fn create_range_iter(start: i32, end: i32) -> std::ops::Range<i32> {
    start..end
}

/// 範囲イテレータ（終端を含む）を生成します。
pub fn create_range_inclusive_iter(start: i32, end: i32) -> std::ops::RangeInclusive<i32> {
    start..=end
}

/// イテレータの要素にインデックスを付けます。
pub fn enumerate_with_index<T>(data: &[T]) -> Vec<(usize, &T)> {
    data.iter().enumerate().collect()
}

/// 2つのイテレータをペアにします。
pub fn zip_two_iterators<T, U>(first: &[T], second: &[U]) -> Vec<(&T, &U)> {
    first.iter().zip(second.iter()).collect()
}

/// 2つのイテレータを連結します。
pub fn chain_iterators<T: Clone>(first: &[T], second: &[T]) -> Vec<T> {
    first.iter().chain(second.iter()).cloned().collect()
}

/// イテレータの各要素を2倍にします。
pub fn map_double(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().map(|x| x * 2).collect()
}

/// イテレータの各要素を文字列に変換します。
pub fn map_to_string<T: std::fmt::Display>(data: &[T]) -> Vec<String> {
    data.iter().map(|x| x.to_string()).collect()
}

/// 文字列のイテレータを大文字に変換します。
pub fn map_to_uppercase(strings: &[&str]) -> Vec<String> {
    strings.iter().map(|s| s.to_uppercase()).collect()
}

/// ネストしたイテレータをフラットにします。
pub fn flat_map_nested<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flat_map(|v| v.iter()).cloned().collect()
}

/// `Option`のイテレータをフラットにし、`Some`の値のみを収集します。
pub fn flatten_options<T>(options: Vec<Option<T>>) -> Vec<T> {
    options.into_iter().flatten().collect()
}

/// `Result`のイテレータをフラットにし、`Ok`の値のみを収集します。
pub fn flatten_results<T, E>(results: Vec<Result<T, E>>) -> Vec<T> {
    results.into_iter().filter_map(|r| r.ok()).collect()
}

/// 実行中の合計を計算します（スキャン操作）。
pub fn scan_running_sum(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().scan(0, |acc, x| {
        *acc += x;
        Some(*acc)
    }).collect()
}

/// 実行中の積を計算します（スキャン操作）。
pub fn scan_running_product(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().scan(1, |acc, x| {
        *acc *= x;
        Some(*acc)
    }).collect()
}

/// イテレータの各要素をデバッグ出力します。
pub fn inspect_debug<T: std::fmt::Debug + Clone>(data: &[T]) -> Vec<T> {
    data.iter()
        .inspect(|x| println!("Processing: {:?}", x))
        .cloned()
        .collect()
}

/// イテレータから偶数の要素をフィルタリングします。
pub fn filter_even(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&x| x % 2 == 0).cloned().collect()
}

/// イテレータから奇数の要素をフィルタリングします。
pub fn filter_odd(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&x| x % 2 == 1).cloned().collect()
}

/// しきい値より大きい要素をフィルタリングします。
pub fn filter_greater_than(numbers: &[i32], threshold: i32) -> Vec<i32> {
    numbers.iter().filter(|&x| x > &threshold).cloned().collect()
}

/// 文字列の長さでフィルタリングします。
pub fn filter_by_length(strings: &[String], min_length: usize) -> Vec<String> {
    strings.iter()
        .filter(|s| s.len() >= min_length)
        .cloned()
        .collect()
}

/// 文字列を整数にパースし、成功したものだけを収集します。
pub fn filter_map_parse_int(strings: &[&str]) -> Vec<i32> {
    strings.iter()
        .filter_map(|s| s.parse().ok())
        .collect()
}

/// `filter_map`と`filter`を組み合わせて使用します。
pub fn filter_map_with_condition<T, U, F, P>(data: &[T], transform: F, predicate: P) -> Vec<U>
where
    F: Fn(&T) -> Option<U>,
    P: Fn(&U) -> bool,
{
    data.iter()
        .filter_map(transform)
        .filter(predicate)
        .collect()
}

/// 最初のn個の要素を取得します。
pub fn take_first_n<T: Clone>(data: &[T], n: usize) -> Vec<T> {
    data.iter().take(n).cloned().collect()
}

/// 条件を満たす間、要素を取得します。
pub fn take_while_condition<T: Clone, F>(data: &[T], condition: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    data.iter().take_while(condition).cloned().collect()
}

/// 最初のn個の要素をスキップします。
pub fn skip_first_n<T: Clone>(data: &[T], n: usize) -> Vec<T> {
    data.iter().skip(n).cloned().collect()
}

/// 条件を満たす間、要素をスキップします。
pub fn skip_while_condition<T: Clone, F>(data: &[T], condition: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    data.iter().skip_while(condition).cloned().collect()
}

/// n個ごとに要素を取得します。
pub fn step_by_n<T: Clone>(data: &[T], step: usize) -> Vec<T> {
    data.iter().step_by(step).cloned().collect()
}

/// イテレータから重複する要素を除きます。
pub fn unique_elements<T: Clone + std::hash::Hash + Eq>(data: &[T]) -> Vec<T> {
    let mut seen = std::collections::HashSet::new();
    data.iter()
        .filter(|&x| seen.insert(x.clone()))
        .cloned()
        .collect()
}

/// イテレータの要素を合計します（`reduce`使用）。
pub fn reduce_sum(numbers: &[i32]) -> Option<i32> {
    numbers.iter().reduce(|acc, x| acc + x).copied()
}

/// イテレータの最大値を検索します。
pub fn reduce_max<T: Ord + Clone>(data: &[T]) -> Option<T> {
    data.iter().max().cloned()
}

/// イテレータの最小値を検索します。
pub fn reduce_min<T: Ord + Clone>(data: &[T]) -> Option<T> {
    data.iter().min().cloned()
}

/// イテレータの要素を合計します（`fold`使用）。
pub fn fold_sum(numbers: &[i32]) -> i32 {
    numbers.iter().fold(0, |acc, x| acc + x)
}

/// イテレータの要素の積を計算します。
pub fn fold_product(numbers: &[i32]) -> i32 {
    numbers.iter().fold(1, |acc, x| acc * x)
}

/// 文字列のイテレータを連結します。
pub fn fold_concat(strings: &[String]) -> String {
    strings.iter().fold(String::new(), |acc, s| acc + s)
}

/// 文字列を区切り文字で連結します。
pub fn fold_with_separator(strings: &[String], separator: &str) -> String {
    strings.iter()
        .fold(String::new(), |acc, s| {
            if acc.is_empty() {
                s.clone()
            } else {
                acc + separator + s
            }
        })
}

/// イテレータから`Vec`を生成します。
pub fn collect_to_vec<T: Clone>(iter: impl Iterator<Item = T>) -> Vec<T> {
    iter.collect()
}

/// キーと値のペアから`HashMap`を生成します。
pub fn collect_to_hashmap<K, V>(pairs: &[(K, V)]) -> HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    pairs.iter().cloned().collect()
}

/// 文字のイテレータから`String`を生成します。
pub fn collect_to_string(chars: &[char]) -> String {
    chars.iter().collect()
}

/// 条件に基づいてイテレータを2つに分割します。
pub fn partition_by_condition<T: Clone, F>(data: &[T], condition: F) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
{
    data.iter().cloned().partition(condition)
}

/// キーに基づいて要素をグループ化します。
pub fn group_by_key<T, K, F>(data: &[T], key_fn: F) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: std::hash::Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut groups = HashMap::new();
    for item in data {
        let key = key_fn(item);
        groups.entry(key).or_default().push(item.clone());
    }
    groups
}

/// イテレータの要素の合計を計算します（`sum`使用）。
pub fn sum_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

/// イテレータの要素の積を計算します（`product`使用）。
pub fn product_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().product()
}

/// イテレータの要素数をカウントします。
pub fn count_elements<T>(data: &[T]) -> usize {
    data.iter().count()
}

/// 条件に最初に一致する要素を検索します。
pub fn find_first_match<T, F>(data: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    data.iter().find(predicate)
}

/// 条件に一致する最初の要素の位置を検索します。
pub fn find_position<T, F>(data: &[T], predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    data.iter().position(predicate)
}

/// 条件に一致する最後の要素の位置を検索します。
pub fn find_last_position<T, F>(data: &[T], predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    data.iter().rposition(predicate)
}

/// イテレータをn回繰り返します。
pub fn cycle_n_times<T: Clone>(data: &[T], n: usize) -> Vec<T> {
    data.iter().cycle().take(n).cloned().collect()
}

/// 特定の値をn回繰り返すイテレータを生成します。
pub fn repeat_value<T: Clone>(value: T, n: usize) -> Vec<T> {
    std::iter::repeat(value).take(n).collect()
}

/// 関数を使って値を生成し、n回繰り返します。
pub fn repeat_with_function<T, F>(generator: F, n: usize) -> Vec<T>
where
    F: Fn() -> T,
{
    std::iter::repeat_with(generator).take(n).collect()
}

/// フィボナッチ数列を生成します。
pub fn generate_fibonacci(n: usize) -> Vec<u64> {
    std::iter::successors(Some((0, 1)), |&(a, b)| Some((b, a + b)))
        .take(n)
        .map(|(a, _)| a)
        .collect()
}

/// 2のべき乗を生成します。
pub fn generate_powers_of_two(n: usize) -> Vec<u64> {
    std::iter::successors(Some(1u64), |&x| Some(x * 2))
        .take(n)
        .collect()
}

/// イテレータを逆順にします。
pub fn reverse_order<T: Clone>(data: &[T]) -> Vec<T> {
    data.iter().rev().cloned().collect()
}

/// 範囲イテレータを逆順にします。
pub fn reverse_range(start: i32, end: i32) -> Vec<i32> {
    (start..=end).rev().collect()
}

/// `peekable`を使用して次の要素をプレビューします。
pub fn peekable_example<T: Clone + std::fmt::Debug>(data: &[T]) -> Vec<T> {
    let mut iter = data.iter().peekable();
    let mut result = Vec::new();
    
    while let Some(&value) = iter.peek() {
        println!("Next value: {:?}", value);
        result.push(value.clone());
        iter.next();
    }
    
    result
}

/// スライディングウィンドウを生成します。
pub fn sliding_windows<T: Clone>(data: &[T], window_size: usize) -> Vec<Vec<T>> {
    data.windows(window_size)
        .map(|w| w.to_vec())
        .collect()
}

/// 指定されたサイズのチャンクに分割します。
pub fn chunks_of_size<T: Clone>(data: &[T], chunk_size: usize) -> Vec<Vec<T>> {
    data.chunks(chunk_size)
        .map(|c| c.to_vec())
        .collect()
}

/// 指定されたサイズの正確なチャンクに分割します（余りは無視）。
pub fn exact_chunks<T: Clone>(data: &[T], chunk_size: usize) -> Vec<Vec<T>> {
    data.chunks_exact(chunk_size)
        .map(|c| c.to_vec())
        .collect()
}

/// エラーが発生する可能性のある`fold`操作。
pub fn try_fold_with_error<T, E, F>(data: &[T], init: i32, f: F) -> Result<i32, E>
where
    F: Fn(i32, &T) -> Result<i32, E>,
{
    data.iter().try_fold(init, f)
}

/// エラーが発生する可能性のある`for_each`操作。
pub fn try_for_each_with_error<T, E, F>(data: &[T], f: F) -> Result<(), E>
where
    F: Fn(&T) -> Result<(), E>,
{
    data.iter().try_for_each(f)
}

/// 条件に一致する要素が1つでも存在するかチェックします。
pub fn any_match<T, F>(data: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    data.iter().any(predicate)
}

/// 全ての要素が条件に一致するかチェックします。
pub fn all_match<T, F>(data: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    data.iter().all(predicate)
}

/// 条件に一致する要素が存在しないかチェックします。
pub fn none_match<T, F>(data: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    !data.iter().any(predicate)
}

/// 各要素に対して副作用のある操作を適用します。
pub fn for_each_with_side_effect<T, F>(data: &[T], f: F)
where
    F: Fn(&T),
{
    data.iter().for_each(f);
}

/// インデックス付きで各要素に副作用のある操作を適用します。
pub fn for_each_with_index<T, F>(data: &[T], f: F)
where
    F: Fn(usize, &T),
{
    data.iter().enumerate().for_each(|(i, x)| f(i, x));
}

/// 0から`max-1`までカウントするカスタムイテレータ。
pub struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    /// 新しい`Counter`を生成します。
    pub fn new(max: usize) -> Self {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;

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

/// `Counter`イテレータを生成します。
pub fn create_counter(max: usize) -> Counter {
    Counter::new(max)
}

/// クロージャからイテレータを生成します。
pub fn from_fn_generator<T, F>(generator: F) -> impl Iterator<Item = T>
where
    F: FnMut() -> Option<T>,
{
    std::iter::from_fn(generator)
}

/// 単一の要素を持つイテレータを生成します。
pub fn once_value<T>(value: T) -> impl Iterator<Item = T> {
    std::iter::once(value)
}

/// 空のイテレータを生成します。
pub fn empty_iterator<T>() -> impl Iterator<Item = T> {
    std::iter::empty()
}

/// CSV形式の行をパースします。
pub fn parse_csv_line(line: &str) -> Vec<&str> {
    line.split(',').collect()
}

/// 文字列スライスから数値をフィルタリングしてパースします。
pub fn filter_and_parse_numbers(strings: &[&str]) -> Vec<i32> {
    strings.iter()
        .filter_map(|s| s.parse().ok())
        .collect()
}

/// テキスト内の単語の出現回数をカウントします。
pub fn word_count(text: &str) -> HashMap<String, usize> {
    text.split_whitespace()
        .map(|word| word.to_lowercase())
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}

/// データ内で最も頻繁に出現する上位n個の要素を取得します。
pub fn top_n_frequent<T>(data: &[T], n: usize) -> Vec<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut freq = HashMap::new();
    for item in data {
        *freq.entry(item.clone()).or_insert(0) += 1;
    }
    
    let mut items: Vec<_> = freq.into_iter().collect();
    items.sort_by(|a, b| b.1.cmp(&a.1));
    
    items.into_iter().take(n).map(|(item, _)| item).collect()
}

/// 移動平均を計算します。
pub fn moving_average(numbers: &[f64], window_size: usize) -> Vec<f64> {
    numbers.windows(window_size)
        .map(|window| window.iter().sum::<f64>() / window.len() as f64)
        .collect()
}

/// 行列を転置します。
pub fn transpose_matrix<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }
    
    let rows = matrix.len();
    let cols = matrix[0].len();
    
    (0..cols)
        .map(|col| {
            (0..rows)
                .map(|row| matrix[row][col].clone())
                .collect()
        })
        .collect()
}

/// 2つのイテレータの要素を交互に配置します。
pub fn interleave<T: Clone>(first: &[T], second: &[T]) -> Vec<T> {
    first.iter()
        .zip(second.iter())
        .flat_map(|(a, b)| vec![a.clone(), b.clone()])
        .collect()
}

/// データをバッチ処理します。
pub fn batch_process<T, U, F>(data: &[T], batch_size: usize, processor: F) -> Vec<U>
where
    F: Fn(&[T]) -> U,
{
    data.chunks(batch_size)
        .map(processor)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_double() {
        let input = vec![1, 2, 3, 4, 5];
        let result = map_double(&input);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_filter_even() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let result = filter_even(&input);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_fold_sum() {
        let input = vec![1, 2, 3, 4, 5];
        let result = fold_sum(&input);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_unique_elements() {
        let input = vec![1, 2, 2, 3, 3, 3, 4];
        let result = unique_elements(&input);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_counter() {
        let counter = create_counter(5);
        let result: Vec<_> = counter.collect();
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_partition_by_condition() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let (evens, odds) = partition_by_condition(&input, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_sliding_windows() {
        let input = vec![1, 2, 3, 4, 5];
        let result = sliding_windows(&input, 3);
        assert_eq!(result, vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
    }

    #[test]
    fn test_generate_fibonacci() {
        let result = generate_fibonacci(10);
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}