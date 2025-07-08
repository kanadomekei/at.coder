use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap, BTreeMap, BTreeSet};
use std::cmp::Reverse;

// HashMap の基本操作
pub fn hashmap_examples() {
    let mut map = hashmap_create();
    hashmap_insert(&mut map);
    hashmap_get(&map);
    hashmap_contains(&map);
    hashmap_update(&mut map);
    hashmap_entry_api(&mut map);
    hashmap_remove(&mut map);
    hashmap_iterate(&map);
    hashmap_iterate_keys(&map);
    hashmap_iterate_values(&map);
}

// HashMapを作成
pub fn hashmap_create() -> HashMap<String, i32> {
    HashMap::new()
}

// HashMap に値を挿入
pub fn hashmap_insert(map: &mut HashMap<String, i32>) {
    map.insert("apple".to_string(), 100);
    map.insert("banana".to_string(), 200);
    map.insert("cherry".to_string(), 300);
}

// HashMap から値を取得
pub fn hashmap_get(map: &HashMap<String, i32>) {
    if let Some(value) = map.get("apple") {
        println!("apple: {}", value);
    }
}

// HashMap でキーの存在確認
pub fn hashmap_contains(map: &HashMap<String, i32>) {
    if map.contains_key("apple") {
        println!("apple exists");
    }
}

// HashMap の値を更新
pub fn hashmap_update(map: &mut HashMap<String, i32>) {
    map.insert("apple".to_string(), 150);
}

// HashMap の entry API を使った更新
pub fn hashmap_entry_api(map: &mut HashMap<String, i32>) {
    map.entry("orange".to_string()).or_insert(250);
    *map.entry("apple".to_string()).or_insert(0) += 50;
}

// HashMap から要素を削除
pub fn hashmap_remove(map: &mut HashMap<String, i32>) {
    map.remove("banana");
}

// HashMap の全要素を反復処理
pub fn hashmap_iterate(map: &HashMap<String, i32>) {
    for (key, value) in map {
        println!("{}: {}", key, value);
    }
}

// HashMap のキーのみを反復処理
pub fn hashmap_iterate_keys(map: &HashMap<String, i32>) {
    for key in map.keys() {
        println!("key: {}", key);
    }
}

// HashMap の値のみを反復処理
pub fn hashmap_iterate_values(map: &HashMap<String, i32>) {
    for value in map.values() {
        println!("value: {}", value);
    }
}

// HashMap の高度な操作
pub fn hashmap_advanced() {
    let mut counter = hashmap_char_counter();
    hashmap_frequency_count(&mut counter, "hello world");
    hashmap_find_max_entry(&counter);
    hashmap_retain_condition(&mut counter);
    hashmap_merge(&mut counter);
}

// 文字カウンターの作成
pub fn hashmap_char_counter() -> HashMap<char, i32> {
    HashMap::new()
}

// 文字の頻度をカウント
pub fn hashmap_frequency_count(counter: &mut HashMap<char, i32>, text: &str) {
    for ch in text.chars() {
        if ch != ' ' {
            *counter.entry(ch).or_insert(0) += 1;
        }
    }
}

// 最大値を持つエントリを取得
pub fn hashmap_find_max_entry(counter: &HashMap<char, i32>) {
    if let Some((max_char, max_count)) = counter.iter().max_by_key(|&(_, count)| count) {
        println!("Most frequent char: '{}' appears {} times", max_char, max_count);
    }
}

// 条件に基づいて要素を保持
pub fn hashmap_retain_condition(counter: &mut HashMap<char, i32>) {
    counter.retain(|&_key, &mut value| value > 1);
}

// 別のHashMapとマージ
pub fn hashmap_merge(counter: &mut HashMap<char, i32>) {
    let mut other_map = HashMap::new();
    other_map.insert('x', 5);
    other_map.insert('y', 10);
    
    for (key, value) in other_map {
        *counter.entry(key).or_insert(0) += value;
    }
}

// HashSet の基本操作
pub fn hashset_examples() {
    let mut set = hashset_create();
    hashset_insert(&mut set);
    hashset_contains(&set);
    hashset_remove(&mut set);
    hashset_iterate(&set);
    hashset_set_operations();
}

// HashSetを作成
pub fn hashset_create() -> HashSet<i32> {
    HashSet::new()
}

// HashSetに値を挿入
pub fn hashset_insert(set: &mut HashSet<i32>) {
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(2); // 重複は無視される
}

// HashSetで値の存在確認
pub fn hashset_contains(set: &HashSet<i32>) {
    if set.contains(&2) {
        println!("2 exists in set");
    }
}

// HashSetから値を削除
pub fn hashset_remove(set: &mut HashSet<i32>) {
    set.remove(&2);
}

// HashSetの全要素を反復処理
pub fn hashset_iterate(set: &HashSet<i32>) {
    for value in set {
        println!("value: {}", value);
    }
}

// HashSetの集合演算
pub fn hashset_set_operations() {
    let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
    
    hashset_union(&set1, &set2);
    hashset_intersection(&set1, &set2);
    hashset_difference(&set1, &set2);
    hashset_symmetric_difference(&set1, &set2);
}

// 和集合
pub fn hashset_union(set1: &HashSet<i32>, set2: &HashSet<i32>) {
    let union: HashSet<i32> = set1.union(set2).cloned().collect();
    println!("Union: {:?}", union);
}

// 積集合
pub fn hashset_intersection(set1: &HashSet<i32>, set2: &HashSet<i32>) {
    let intersection: HashSet<i32> = set1.intersection(set2).cloned().collect();
    println!("Intersection: {:?}", intersection);
}

// 差集合
pub fn hashset_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) {
    let difference: HashSet<i32> = set1.difference(set2).cloned().collect();
    println!("Difference: {:?}", difference);
}

// 対称差集合
pub fn hashset_symmetric_difference(set1: &HashSet<i32>, set2: &HashSet<i32>) {
    let symmetric_difference: HashSet<i32> = set1.symmetric_difference(set2).cloned().collect();
    println!("Symmetric difference: {:?}", symmetric_difference);
}

// HashSet の高度な操作
pub fn hashset_advanced() {
    let numbers = vec![1, 2, 3, 4, 5, 2, 3, 6, 7, 8, 1];
    
    let unique_numbers = hashset_remove_duplicates(&numbers);
    hashset_check_duplicates(&numbers, &unique_numbers);
    hashset_filter_even(&unique_numbers);
}

// 重複を削除
pub fn hashset_remove_duplicates(numbers: &[i32]) -> HashSet<i32> {
    let unique_numbers: HashSet<i32> = numbers.iter().cloned().collect();
    println!("Unique numbers: {:?}", unique_numbers);
    unique_numbers
}

// 重複があるかチェック
pub fn hashset_check_duplicates(numbers: &[i32], unique_numbers: &HashSet<i32>) {
    let has_duplicates = numbers.len() != unique_numbers.len();
    println!("Has duplicates: {}", has_duplicates);
}

// 条件に基づいて要素をフィルタリング
pub fn hashset_filter_even(unique_numbers: &HashSet<i32>) {
    let mut filtered_set = unique_numbers.clone();
    filtered_set.retain(|&x| x % 2 == 0);
    println!("Even numbers: {:?}", filtered_set);
}

// Tuple の基本操作
pub fn tuple_examples() {
    tuple_basic();
    tuple_destructure();
    tuple_mixed_types();
    tuple_nested();
    tuple_single_element();
}

// 基本的なタプル
pub fn tuple_basic() {
    let point = (3, 4);
    println!("Point: ({}, {})", point.0, point.1);
}

// タプルの分解
pub fn tuple_destructure() {
    let point = (3, 4);
    let (x, y) = point;
    println!("x: {}, y: {}", x, y);
}

// 異なる型のタプル
pub fn tuple_mixed_types() {
    let person = ("Alice", 30, true);
    let (name, age, is_student) = person;
    println!("Name: {}, Age: {}, Student: {}", name, age, is_student);
}

// ネストしたタプル
pub fn tuple_nested() {
    let nested = ((1, 2), (3, 4));
    println!("Nested: {:?}", nested);
}

// 単一要素のタプル
pub fn tuple_single_element() {
    let single = (42,);
    println!("Single element tuple: {:?}", single);
}

// Tuple の高度な操作
pub fn tuple_advanced() {
    tuple_distance_calculation();
    tuple_max_point();
    tuple_sorting();
    tuple_multi_key_sorting();
}

// 座標間の距離計算
pub fn tuple_distance_calculation() {
    let point1 = (3.0, 4.0);
    let point2 = (6.0, 8.0);
    
    let distance = ((point2.0 - point1.0).powi(2) + (point2.1 - point1.1).powi(2)).sqrt();
    println!("Distance: {}", distance);
}

// 合計が最大の点を取得
pub fn tuple_max_point() {
    let points = [(0, 0), (1, 1), (2, 2), (3, 3)];
    
    if let Some(max_point) = points.iter().max_by_key(|&&(x, y)| x + y) {
        println!("Max point: {:?}", max_point);
    }
}

// タプルを合計でソート
pub fn tuple_sorting() {
    let mut points_vec = vec![(3, 1), (1, 3), (2, 2), (1, 1)];
    points_vec.sort_by_key(|&(x, y)| x + y);
    println!("Sorted points: {:?}", points_vec);
}

// タプルを複数キーでソート
pub fn tuple_multi_key_sorting() {
    let mut points_vec = vec![(3, 1), (1, 3), (2, 2), (1, 1)];
    points_vec.sort_by(|&(x1, y1), &(x2, y2)| {
        x1.cmp(&x2).then(y1.cmp(&y2))
    });
    println!("Sorted by x then y: {:?}", points_vec);
}

// 組み合わせ例: HashMap + Set + Tuple
pub fn combined_example() {
    // グラフの隣接リスト表現
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    
    // エッジを追加
    let edges = [(1, 2), (1, 3), (2, 4), (3, 4), (4, 5)];
    
    for (from, to) in edges {
        graph.entry(from).or_insert_with(HashSet::new).insert(to);
        graph.entry(to).or_insert_with(HashSet::new).insert(from);
    }
    
    // グラフを表示
    for (node, neighbors) in &graph {
        println!("Node {}: {:?}", node, neighbors);
    }
    
    // 特定ノードの隣接数を取得
    if let Some(neighbors) = graph.get(&1) {
        println!("Node 1 has {} neighbors", neighbors.len());
    }
    
    // 全ノードの次数を計算
    let degrees: HashMap<i32, usize> = graph.iter()
        .map(|(node, neighbors)| (*node, neighbors.len()))
        .collect();
    
    println!("Node degrees: {:?}", degrees);
}

// 頻度カウンター
pub fn frequency_counter<T: std::hash::Hash + Eq + Clone>(items: &[T]) -> HashMap<T, usize> {
    let mut counter = HashMap::new();
    for item in items {
        *counter.entry(item.clone()).or_insert(0) += 1;
    }
    counter
}

// 集合演算のヘルパー関数
pub fn set_operations<T: std::hash::Hash + Eq + Clone>(
    set1: &HashSet<T>, 
    set2: &HashSet<T>
) -> (HashSet<T>, HashSet<T>, HashSet<T>, HashSet<T>) {
    let union = set1.union(set2).cloned().collect();
    let intersection = set1.intersection(set2).cloned().collect();
    let difference = set1.difference(set2).cloned().collect();
    let symmetric_difference = set1.symmetric_difference(set2).cloned().collect();
    
    (union, intersection, difference, symmetric_difference)
}

// タプルのユーティリティ関数
pub fn tuple_to_vec<T: Clone>(tuple: (T, T)) -> Vec<T> {
    vec![tuple.0, tuple.1]
}

pub fn vec_to_tuple<T: Clone>(vec: Vec<T>) -> Option<(T, T)> {
    if vec.len() == 2 {
        Some((vec[0].clone(), vec[1].clone()))
    } else {
        None
    }
}

// VecDeque（両端キュー）の基本操作
pub fn vecdeque_examples() {
    let mut deque = vecdeque_create();
    vecdeque_push_back(&mut deque);
    vecdeque_push_front(&mut deque);
    vecdeque_display(&deque);
    vecdeque_pop_front(&mut deque);
    vecdeque_pop_back(&mut deque);
    vecdeque_front_back_ref(&deque);
    vecdeque_index_access(&deque);
}

// VecDequeを作成
pub fn vecdeque_create() -> VecDeque<i32> {
    VecDeque::new()
}

// VecDequeの後端に追加
pub fn vecdeque_push_back(deque: &mut VecDeque<i32>) {
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
}

// VecDequeの前端に追加
pub fn vecdeque_push_front(deque: &mut VecDeque<i32>) {
    deque.push_front(0);
    deque.push_front(-1);
}

// VecDequeを表示
pub fn vecdeque_display(deque: &VecDeque<i32>) {
    println!("Deque: {:?}", deque);
}

// VecDequeの前端から取得・削除
pub fn vecdeque_pop_front(deque: &mut VecDeque<i32>) {
    if let Some(front) = deque.pop_front() {
        println!("Front: {}", front);
    }
}

// VecDequeの後端から取得・削除
pub fn vecdeque_pop_back(deque: &mut VecDeque<i32>) {
    if let Some(back) = deque.pop_back() {
        println!("Back: {}", back);
    }
}

// VecDequeの先頭・末尾の参照
pub fn vecdeque_front_back_ref(deque: &VecDeque<i32>) {
    if let Some(front) = deque.front() {
        println!("Front ref: {}", front);
    }
    if let Some(back) = deque.back() {
        println!("Back ref: {}", back);
    }
}

// VecDequeのインデックスアクセス
pub fn vecdeque_index_access(deque: &VecDeque<i32>) {
    if let Some(value) = deque.get(1) {
        println!("Index 1: {}", value);
    }
}

// BinaryHeap（優先度付きキュー）の基本操作
pub fn binaryheap_examples() {
    binaryheap_max_heap();
    binaryheap_min_heap();
}

// 最大ヒープの作成と操作
pub fn binaryheap_max_heap() {
    let mut max_heap = BinaryHeap::new();
    
    binaryheap_push_max(&mut max_heap);
    binaryheap_display_max(&max_heap);
    binaryheap_pop_max(&mut max_heap);
}

// 最小ヒープの作成と操作
pub fn binaryheap_min_heap() {
    let mut min_heap = BinaryHeap::new();
    
    binaryheap_push_min(&mut min_heap);
    binaryheap_display_min(&min_heap);
    binaryheap_pop_min(&mut min_heap);
}

// 最大ヒープに要素を追加
pub fn binaryheap_push_max(max_heap: &mut BinaryHeap<i32>) {
    max_heap.push(3);
    max_heap.push(1);
    max_heap.push(4);
    max_heap.push(2);
}

// 最大ヒープを表示
pub fn binaryheap_display_max(max_heap: &BinaryHeap<i32>) {
    println!("Max heap: {:?}", max_heap);
}

// 最大ヒープから最大値を取得・削除
pub fn binaryheap_pop_max(max_heap: &mut BinaryHeap<i32>) {
    let mut heap_copy = max_heap.clone();
    while let Some(max) = heap_copy.pop() {
        println!("Max: {}", max);
    }
}

// 最小ヒープに要素を追加
pub fn binaryheap_push_min(min_heap: &mut BinaryHeap<Reverse<i32>>) {
    min_heap.push(Reverse(3));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(4));
    min_heap.push(Reverse(2));
}

// 最小ヒープを表示
pub fn binaryheap_display_min(min_heap: &BinaryHeap<Reverse<i32>>) {
    println!("Min heap: {:?}", min_heap);
}

// 最小ヒープから最小値を取得・削除
pub fn binaryheap_pop_min(min_heap: &mut BinaryHeap<Reverse<i32>>) {
    let mut heap_copy = min_heap.clone();
    while let Some(Reverse(min)) = heap_copy.pop() {
        println!("Min: {}", min);
    }
}

// BTreeMap（順序付きマップ）の基本操作
pub fn btreemap_examples() {
    let mut map = btreemap_create();
    btreemap_insert(&mut map);
    btreemap_iterate(&map);
    btreemap_range_query(&map);
    btreemap_first_last(&map);
}

// BTreeMapを作成
pub fn btreemap_create() -> BTreeMap<i32, String> {
    BTreeMap::new()
}

// BTreeMapに値を挿入
pub fn btreemap_insert(map: &mut BTreeMap<i32, String>) {
    map.insert(3, "three".to_string());
    map.insert(1, "one".to_string());
    map.insert(4, "four".to_string());
    map.insert(2, "two".to_string());
}

// BTreeMapをキー順で反復処理
pub fn btreemap_iterate(map: &BTreeMap<i32, String>) {
    for (key, value) in map {
        println!("{}: {}", key, value);
    }
}

// BTreeMapの範囲検索
pub fn btreemap_range_query(map: &BTreeMap<i32, String>) {
    for (key, value) in map.range(2..=3) {
        println!("Range [2,3]: {}: {}", key, value);
    }
}

// BTreeMapの最初と最後の要素
pub fn btreemap_first_last(map: &BTreeMap<i32, String>) {
    if let Some((first_key, first_value)) = map.first_key_value() {
        println!("First: {}: {}", first_key, first_value);
    }
    if let Some((last_key, last_value)) = map.last_key_value() {
        println!("Last: {}: {}", last_key, last_value);
    }
}

// BTreeSet（順序付き集合）の基本操作
pub fn btreeset_examples() {
    let mut set = btreeset_create();
    btreeset_insert(&mut set);
    btreeset_iterate(&set);
    btreeset_range_query(&set);
    btreeset_first_last(&set);
}

// BTreeSetを作成
pub fn btreeset_create() -> BTreeSet<i32> {
    BTreeSet::new()
}

// BTreeSetに値を挿入
pub fn btreeset_insert(set: &mut BTreeSet<i32>) {
    set.insert(3);
    set.insert(1);
    set.insert(4);
    set.insert(2);
}

// BTreeSetを順序で反復処理
pub fn btreeset_iterate(set: &BTreeSet<i32>) {
    for value in set {
        println!("Value: {}", value);
    }
}

// BTreeSetの範囲検索
pub fn btreeset_range_query(set: &BTreeSet<i32>) {
    for value in set.range(2..=3) {
        println!("Range [2,3]: {}", value);
    }
}

// BTreeSetの最初と最後の要素
pub fn btreeset_first_last(set: &BTreeSet<i32>) {
    if let Some(first) = set.first() {
        println!("First: {}", first);
    }
    if let Some(last) = set.last() {
        println!("Last: {}", last);
    }
}

// Vector（動的配列）の高度な操作
pub fn vector_advanced() {
    let mut vec = vec![1, 2, 3, 4, 5];
    
    vector_insert(&mut vec);
    vector_remove(&mut vec);
    vector_drain(&mut vec);
    vector_retain(&mut vec);
    vector_dedup(&mut vec);
    vector_binary_search(&vec);
}

// Vectorに挿入
pub fn vector_insert(vec: &mut Vec<i32>) {
    vec.insert(2, 10);
    println!("After insert: {:?}", vec);
}

// Vectorから削除
pub fn vector_remove(vec: &mut Vec<i32>) {
    let removed = vec.remove(2);
    println!("Removed: {}, Vec: {:?}", removed, vec);
}

// Vectorの範囲削除
pub fn vector_drain(vec: &mut Vec<i32>) {
    vec.drain(1..3);
    println!("After drain: {:?}", vec);
}

// Vectorの条件による保持
pub fn vector_retain(vec: &mut Vec<i32>) {
    vec.retain(|&x| x % 2 == 0);
    println!("Even numbers: {:?}", vec);
}

// Vectorの重複削除
pub fn vector_dedup(vec: &mut Vec<i32>) {
    vec.extend([2, 4, 6, 2, 4]);
    vec.sort();
    vec.dedup();
    println!("After dedup: {:?}", vec);
}

// Vectorの二分探索
pub fn vector_binary_search(vec: &Vec<i32>) {
    let target = 4;
    match vec.binary_search(&target) {
        Ok(index) => println!("Found {} at index {}", target, index),
        Err(index) => println!("{} should be inserted at index {}", target, index),
    }
}

// Union-Find（素集合データ構造）
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    // 新しいUnionFindを作成
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    
    // 根を見つける（経路圧縮付き）
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    // 二つの集合を結合
    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x != root_y {
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
    
    // 同じ集合に属するかチェック
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

// Union-Findの使用例
pub fn unionfind_example() {
    let mut uf = unionfind_create(5);
    unionfind_union_operations(&mut uf);
    unionfind_connectivity_check(&mut uf);
}

// Union-Findを作成
pub fn unionfind_create(n: usize) -> UnionFind {
    UnionFind::new(n)
}

// Union-Findの結合操作
pub fn unionfind_union_operations(uf: &mut UnionFind) {
    uf.union(0, 1);
    uf.union(2, 3);
    uf.union(1, 2);
}

// Union-Findの接続チェック
pub fn unionfind_connectivity_check(uf: &mut UnionFind) {
    println!("0 and 3 connected: {}", uf.connected(0, 3));
    println!("0 and 4 connected: {}", uf.connected(0, 4));
}

// セグメント木（区間和クエリ）
pub struct SegmentTree {
    tree: Vec<i64>,
    n: usize,
}

impl SegmentTree {
    // 新しいセグメント木を作成
    pub fn new(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut tree = vec![0; 4 * n];
        let mut seg_tree = SegmentTree { tree, n };
        seg_tree.build(arr, 0, 0, n - 1);
        seg_tree
    }
    
    // 構築
    fn build(&mut self, arr: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = arr[start];
        } else {
            let mid = (start + end) / 2;
            self.build(arr, 2 * node + 1, start, mid);
            self.build(arr, 2 * node + 2, mid + 1, end);
            self.tree[node] = self.tree[2 * node + 1] + self.tree[2 * node + 2];
        }
    }
    
    // 区間和クエリ
    pub fn query(&self, l: usize, r: usize) -> i64 {
        self.query_util(0, 0, self.n - 1, l, r)
    }
    
    fn query_util(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        if r < start || end < l {
            return 0;
        }
        if l <= start && end <= r {
            return self.tree[node];
        }
        let mid = (start + end) / 2;
        let left_sum = self.query_util(2 * node + 1, start, mid, l, r);
        let right_sum = self.query_util(2 * node + 2, mid + 1, end, l, r);
        left_sum + right_sum
    }
    
    // 単一要素の更新
    pub fn update(&mut self, idx: usize, val: i64) {
        self.update_util(0, 0, self.n - 1, idx, val);
    }
    
    fn update_util(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i64) {
        if start == end {
            self.tree[node] = val;
        } else {
            let mid = (start + end) / 2;
            if idx <= mid {
                self.update_util(2 * node + 1, start, mid, idx, val);
            } else {
                self.update_util(2 * node + 2, mid + 1, end, idx, val);
            }
            self.tree[node] = self.tree[2 * node + 1] + self.tree[2 * node + 2];
        }
    }
}

// セグメント木の使用例
pub fn segment_tree_example() {
    let arr = vec![1, 3, 5, 7, 9, 11];
    let mut seg_tree = segment_tree_create(&arr);
    segment_tree_query(&seg_tree);
    segment_tree_update(&mut seg_tree);
    segment_tree_query_after_update(&seg_tree);
}

// セグメント木を作成
pub fn segment_tree_create(arr: &[i64]) -> SegmentTree {
    SegmentTree::new(arr)
}

// セグメント木のクエリ
pub fn segment_tree_query(seg_tree: &SegmentTree) {
    println!("Sum of range [1,3]: {}", seg_tree.query(1, 3));
}

// セグメント木の更新
pub fn segment_tree_update(seg_tree: &mut SegmentTree) {
    seg_tree.update(1, 10);
}

// セグメント木の更新後クエリ
pub fn segment_tree_query_after_update(seg_tree: &SegmentTree) {
    println!("Sum of range [1,3] after update: {}", seg_tree.query(1, 3));
}

// Fenwick木（Binary Indexed Tree）
pub struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    // 新しいFenwick木を作成
    pub fn new(n: usize) -> Self {
        FenwickTree {
            tree: vec![0; n + 1],
            n,
        }
    }
    
    // 要素を追加
    pub fn add(&mut self, mut i: usize, delta: i64) {
        i += 1; // 1-indexed
        while i <= self.n {
            self.tree[i] += delta;
            i += i & (!i + 1);
        }
    }
    
    // 前綴和を計算
    pub fn sum(&self, mut i: usize) -> i64 {
        i += 1; // 1-indexed
        let mut result = 0;
        while i > 0 {
            result += self.tree[i];
            i -= i & (!i + 1);
        }
        result
    }
    
    // 区間和を計算
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        if l > 0 {
            self.sum(r) - self.sum(l - 1)
        } else {
            self.sum(r)
        }
    }
}

// Fenwick木の使用例
pub fn fenwick_tree_example() {
    let mut ft = fenwick_tree_create(6);
    fenwick_tree_add_elements(&mut ft);
    fenwick_tree_prefix_sum(&ft);
    fenwick_tree_range_sum(&ft);
}

// Fenwick木を作成
pub fn fenwick_tree_create(n: usize) -> FenwickTree {
    FenwickTree::new(n)
}

// Fenwick木に要素を追加
pub fn fenwick_tree_add_elements(ft: &mut FenwickTree) {
    ft.add(0, 1);
    ft.add(1, 3);
    ft.add(2, 5);
    ft.add(3, 7);
    ft.add(4, 9);
    ft.add(5, 11);
}

// Fenwick木の前綴和
pub fn fenwick_tree_prefix_sum(ft: &FenwickTree) {
    println!("Sum of first 4 elements: {}", ft.sum(3));
}

// Fenwick木の区間和
pub fn fenwick_tree_range_sum(ft: &FenwickTree) {
    println!("Sum of range [1,3]: {}", ft.range_sum(1, 3));
}