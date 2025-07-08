// 1のビット数をカウントする（ポピュレーションカウント）
pub fn count_bits(n: i64) -> i32 {
    n.count_ones() as i32
}

// 指定した位置のビットが1かどうかをチェックする
pub fn is_bit_set(n: i64, pos: i32) -> bool {
    (n & (1 << pos)) != 0
}

// 指定した位置のビットを1にセットする
pub fn set_bit(n: i64, pos: i32) -> i64 {
    n | (1 << pos)
}

// 指定した位置のビットを0にクリアする
pub fn clear_bit(n: i64, pos: i32) -> i64 {
    n & !(1 << pos)
}

// 指定した位置のビットを反転する
pub fn toggle_bit(n: i64, pos: i32) -> i64 {
    n ^ (1 << pos)
}

// 最下位の1ビットを取得する
pub fn get_lowest_bit(n: i64) -> i64 {
    n & -n
}

// 最下位の1ビットをクリアする
pub fn clear_lowest_bit(n: i64) -> i64 {
    n & (n - 1)
}

// ビット順列の次の順列を取得する（同じ1の数で次に大きい数）
pub fn next_permutation(n: i64) -> Option<i64> {
    if n == 0 { return None; }
    
    let c = n & -n;
    let r = n + c;
    
    if r == 0 || (r & (r - 1)) == 0 {
        return None;
    }
    
    Some(r | (((n ^ r) / c) >> 2))
}

// 指定したビットマスクの全てのサブセットを列挙する
pub fn enumerate_subsets(n: i64) -> Vec<i64> {
    let mut result = Vec::new();
    let mut mask = n;
    
    loop {
        result.push(mask);
        if mask == 0 { break; }
        mask = (mask - 1) & n;
    }
    
    result
}

// ビットを逆順にする（指定したビット数で）
pub fn bit_reverse(mut n: i64, bits: i32) -> i64 {
    let mut result = 0;
    for _ in 0..bits {
        result = (result << 1) | (n & 1);
        n >>= 1;
    }
    result
}

// グレイコードを計算する（n番目のグレイコード）
pub fn gray_code(n: i64) -> i64 {
    n ^ (n >> 1)
}

// グレイコードから元の数を復元する
pub fn inverse_gray_code(mut g: i64) -> i64 {
    let mut result = g;
    while g > 0 {
        g >>= 1;
        result ^= g;
    }
    result
}

// 2の累乗かどうかを判定する
pub fn power_of_two(n: i64) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

// n以上の最小の2の累乗を求める
pub fn next_power_of_two(n: i64) -> i64 {
    if n <= 1 { return 1; }
    
    let mut p = 1;
    while p < n {
        p <<= 1;
    }
    p
}

// 最上位ビットの位置を取得する（0-indexed、MSB）
pub fn msb_position(n: i64) -> i32 {
    if n == 0 { return -1; }
    63 - n.leading_zeros() as i32
}

// 最下位ビットの位置を取得する（0-indexed、LSB）
pub fn lsb_position(n: i64) -> i32 {
    if n == 0 { return -1; }
    n.trailing_zeros() as i32
}

// 指定した範囲のビットを抽出する
pub fn extract_bits(n: i64, start: i32, length: i32) -> i64 {
    let mask = (1i64 << length) - 1;
    (n >> start) & mask
}

// 指定した範囲のビットに値を設定する
pub fn set_bits_range(n: i64, start: i32, length: i32, value: i64) -> i64 {
    let mask = (1i64 << length) - 1;
    let cleared = n & !(mask << start);
    cleared | ((value & mask) << start)
}

// 全てのビットを反転する（指定したビット数で）
pub fn flip_all_bits(n: i64, bits: i32) -> i64 {
    let mask = (1i64 << bits) - 1;
    n ^ mask
}

// パリティ（奇偶性）を計算する（1の個数が奇数なら1、偶数なら0）
pub fn parity(n: i64) -> i32 {
    n.count_ones() as i32 % 2
}

// ハミング距離を計算する（異なるビットの数）
pub fn hamming_distance(a: i64, b: i64) -> i32 {
    (a ^ b).count_ones() as i32
}

// 最上位の1ビットのみを残す
pub fn isolate_msb(n: i64) -> i64 {
    if n == 0 { return 0; }
    let msb_pos = msb_position(n);
    1i64 << msb_pos
}

// 最下位の1ビットのみを残す（get_lowest_bitと同じだが名前を明確化）
pub fn isolate_lsb(n: i64) -> i64 {
    n & -n
}

// 連続する1ビットの塊の数を数える
pub fn count_bit_groups(mut n: i64) -> i32 {
    let mut count = 0;
    let mut in_group = false;
    
    while n > 0 {
        if n & 1 == 1 {
            if !in_group {
                count += 1;
                in_group = true;
            }
        } else {
            in_group = false;
        }
        n >>= 1;
    }
    
    count
}

// 最長の連続する1ビットの長さを求める
pub fn longest_bit_sequence(mut n: i64) -> i32 {
    let mut max_length = 0;
    let mut current_length = 0;
    
    while n > 0 {
        if n & 1 == 1 {
            current_length += 1;
            max_length = max_length.max(current_length);
        } else {
            current_length = 0;
        }
        n >>= 1;
    }
    
    max_length
}

// ビット位置のリストを取得する（1になっているビットの位置）
pub fn get_bit_positions(mut n: i64) -> Vec<i32> {
    let mut positions = Vec::new();
    let mut pos = 0;
    
    while n > 0 {
        if n & 1 == 1 {
            positions.push(pos);
        }
        n >>= 1;
        pos += 1;
    }
    
    positions
}

// ビットマスクから組み合わせのインデックスを生成する
pub fn mask_to_combination_index(mask: i64, elements: &[i64]) -> Vec<i64> {
    let mut result = Vec::new();
    let positions = get_bit_positions(mask);
    
    for pos in positions {
        if (pos as usize) < elements.len() {
            result.push(elements[pos as usize]);
        }
    }
    
    result
}

// 全ての2^nビットマスクを生成する
pub fn generate_all_masks(n: i32) -> Vec<i64> {
    let mut masks = Vec::new();
    let total = 1i64 << n;
    
    for i in 0..total {
        masks.push(i);
    }
    
    masks
}

// 指定したビット数の1を持つ全てのマスクを生成する
pub fn generate_masks_with_k_bits(n: i32, k: i32) -> Vec<i64> {
    let mut masks = Vec::new();
    
    fn generate_recursive(current_mask: i64, remaining_bits: i32, start_pos: i32, n: i32, masks: &mut Vec<i64>) {
        if remaining_bits == 0 {
            masks.push(current_mask);
            return;
        }
        
        for pos in start_pos..n {
            generate_recursive(current_mask | (1i64 << pos), remaining_bits - 1, pos + 1, n, masks);
        }
    }
    
    generate_recursive(0, k, 0, n, &mut masks);
    masks
}

// ビットシフトで乗算・除算（2の累乗のみ）
pub fn multiply_by_power_of_two(n: i64, power: i32) -> i64 {
    n << power
}

pub fn divide_by_power_of_two(n: i64, power: i32) -> i64 {
    n >> power
}

// ビットマスクの交集合を計算する（両方で1のビットのみ残す）
pub fn bit_intersection(a: i64, b: i64) -> i64 {
    a & b
}

// ビットマスクの和集合を計算する（どちらかで1のビットを残す）
pub fn bit_union(a: i64, b: i64) -> i64 {
    a | b
}

// ビットマスクの差集合を計算する（aにあってbにないビットを残す）
pub fn bit_difference(a: i64, b: i64) -> i64 {
    a & !b
}

// ビットマスクの対称差集合を計算する（片方のみで1のビットを残す）
pub fn bit_symmetric_difference(a: i64, b: i64) -> i64 {
    a ^ b
}