
// 最大公約数を計算する（ユークリッドの互除法）
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// 最小公倍数を計算する
pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

// 冪乗を計算する（高速冪乗法、時間計算量：O(log exp)）
pub fn pow(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        1
    } else if exp % 2 == 0 {
        let half = pow(base, exp / 2);
        half * half
    } else {
        base * pow(base, exp - 1)
    }
}

// 法演算での冪乗を計算する（mod_pow、時間計算量：O(log exp)）
pub fn mod_pow(base: i64, exp: i64, modulo: i64) -> i64 {
    if exp == 0 {
        1
    } else if exp % 2 == 0 {
        let half = mod_pow(base, exp / 2, modulo);
        (half * half) % modulo
    } else {
        (base * mod_pow(base, exp - 1, modulo)) % modulo
    }
}

// 絶対値を計算する
pub fn abs(x: i64) -> i64 {
    if x < 0 { -x } else { x }
}

// 素数判定（時間計算量：O(√n)）
pub fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    for i in (3..=((n as f64).sqrt() as i64)).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// エラトステネスの篩で指定範囲の素数を列挙する（時間計算量：O(n log log n)）
pub fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

// 階乗を計算する（n!）
pub fn factorial(n: i64) -> i64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// 順列を計算する（nPr = n!/(n-r)!）
pub fn permutation(n: i64, r: i64) -> i64 {
    if r > n || r < 0 {
        0
    } else {
        (n - r + 1..=n).product()
    }
}

// 組合せを計算する（nCr = n!/(r!(n-r)!)）
pub fn combination(n: i64, r: i64) -> i64 {
    if r > n || r < 0 {
        0
    } else if r > n - r {
        combination(n, n - r)
    } else {
        let mut result = 1;
        for i in 0..r {
            result = result * (n - i) / (i + 1);
        }
        result
    }
}

// 拡張ユークリッドの互除法（ax + by = gcd(a,b) の解を求める）
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

// 逆元を計算する（mod m での a の逆元）
pub fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        None
    } else {
        Some(((x % m) + m) % m)
    }
}

// 数の桁数を計算する
pub fn digit_count(mut n: i64) -> i32 {
    if n == 0 {
        return 1;
    }
    
    n = n.abs();
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}

// 数の各桁の和を計算する
pub fn digit_sum(mut n: i64) -> i64 {
    n = n.abs();
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

// 数を逆順にする（例：123 → 321）
pub fn reverse_number(mut n: i64) -> i64 {
    let is_negative = n < 0;
    n = n.abs();
    
    let mut result = 0;
    while n > 0 {
        result = result * 10 + n % 10;
        n /= 10;
    }
    
    if is_negative { -result } else { result }
}

// フィボナッチ数列のn項目を計算する（時間計算量：O(log n)）
pub fn fibonacci(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    
    fn matrix_mult(a: [[i64; 2]; 2], b: [[i64; 2]; 2]) -> [[i64; 2]; 2] {
        [
            [a[0][0] * b[0][0] + a[0][1] * b[1][0], a[0][0] * b[0][1] + a[0][1] * b[1][1]],
            [a[1][0] * b[0][0] + a[1][1] * b[1][0], a[1][0] * b[0][1] + a[1][1] * b[1][1]],
        ]
    }
    
    fn matrix_power(mut matrix: [[i64; 2]; 2], mut n: i64) -> [[i64; 2]; 2] {
        let mut result = [[1, 0], [0, 1]]; // 単位行列
        
        while n > 0 {
            if n % 2 == 1 {
                result = matrix_mult(result, matrix);
            }
            matrix = matrix_mult(matrix, matrix);
            n /= 2;
        }
        
        result
    }
    
    let fib_matrix = [[1, 1], [1, 0]];
    let result_matrix = matrix_power(fib_matrix, n);
    result_matrix[0][1]
}

// 平方根が整数かどうかを判定する
pub fn is_perfect_square(n: i64) -> bool {
    if n < 0 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as i64;
    sqrt_n * sqrt_n == n
}

// 完全数かどうかを判定する（その数の真の約数の和がその数と等しい）
pub fn is_perfect_number(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    
    let mut sum = 1; // 1は必ず約数
    let sqrt_n = (n as f64).sqrt() as i64;
    
    for i in 2..=sqrt_n {
        if n % i == 0 {
            sum += i;
            if i != n / i {
                sum += n / i;
            }
        }
    }
    
    sum == n
}

// 約数の個数を計算する
pub fn divisor_count(n: i64) -> i64 {
    let mut count = 0;
    let sqrt_n = (n as f64).sqrt() as i64;
    
    for i in 1..=sqrt_n {
        if n % i == 0 {
            count += 1;
            if i != n / i {
                count += 1;
            }
        }
    }
    
    count
}

// 約数を全て列挙する
pub fn get_divisors(n: i64) -> Vec<i64> {
    let mut divisors = Vec::new();
    let sqrt_n = (n as f64).sqrt() as i64;
    
    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    
    divisors.sort();
    divisors
}

// 素因数分解を行う
pub fn prime_factorization(mut n: i64) -> Vec<(i64, i32)> {
    let mut factors = Vec::new();
    
    // 2で割り切れる分だけ割る
    if n % 2 == 0 {
        let mut count = 0;
        while n % 2 == 0 {
            count += 1;
            n /= 2;
        }
        factors.push((2, count));
    }
    
    // 3以上の奇数で割る
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            let mut count = 0;
            while n % i == 0 {
                count += 1;
                n /= i;
            }
            factors.push((i, count));
        }
        i += 2;
    }
    
    // 残りが1より大きい場合、それは素数
    if n > 1 {
        factors.push((n, 1));
    }
    
    factors
}

// オイラーのφ関数（トーシェント関数）を計算する
pub fn euler_totient(n: i64) -> i64 {
    let factors = prime_factorization(n);
    let mut result = n;
    
    for (p, _) in factors {
        result = result / p * (p - 1);
    }
    
    result
}

// 最大値を求める
pub fn max(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

// 最小値を求める
pub fn min(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}

// 3つの数の最大値を求める
pub fn max3(a: i64, b: i64, c: i64) -> i64 {
    max(max(a, b), c)
}

// 3つの数の最小値を求める
pub fn min3(a: i64, b: i64, c: i64) -> i64 {
    min(min(a, b), c)
}

// 値を指定した範囲にクランプする
pub fn clamp(value: i64, min_val: i64, max_val: i64) -> i64 {
    if value < min_val {
        min_val
    } else if value > max_val {
        max_val
    } else {
        value
    }
}

// 符号を返す（-1, 0, 1）
pub fn sign(x: i64) -> i64 {
    if x > 0 { 1 } else if x < 0 { -1 } else { 0 }
}

// 2つの数を交換する
pub fn swap(a: &mut i64, b: &mut i64) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

// 範囲内の数値の総和を計算する（等差数列の和）
pub fn range_sum(start: i64, end: i64) -> i64 {
    if start > end {
        return 0;
    }
    (end - start + 1) * (start + end) / 2
}

// 等差数列の第n項を計算する
pub fn arithmetic_sequence(a: i64, d: i64, n: i64) -> i64 {
    a + (n - 1) * d
}

// 等比数列の第n項を計算する
pub fn geometric_sequence(a: i64, r: i64, n: i64) -> i64 {
    a * pow(r, n - 1)
}

// 等比数列の和を計算する（初項a、公比r、項数n）
pub fn geometric_sum(a: i64, r: i64, n: i64) -> i64 {
    if r == 1 {
        a * n
    } else {
        a * (pow(r, n) - 1) / (r - 1)
    }
} 