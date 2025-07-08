
// 文字列を逆順にする
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// 文字列が回文（palindrome）かどうかを判定する
pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

// 文字列内の文字の出現回数をカウントする
pub fn char_count(s: &str) -> std::collections::HashMap<char, usize> {
    use std::collections::HashMap;
    let mut count = HashMap::new();
    
    for ch in s.chars() {
        *count.entry(ch).or_insert(0) += 1;
    }
    
    count
}

// 文字列がアナグラムかどうかを判定する
pub fn is_anagram(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    
    let count1 = char_count(s1);
    let count2 = char_count(s2);
    
    count1 == count2
}

// 文字列から重複文字を除去する（順序を保持）
pub fn remove_duplicates(s: &str) -> String {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut result = String::new();
    
    for ch in s.chars() {
        if seen.insert(ch) {
            result.push(ch);
        }
    }
    
    result
}

// 文字列の最長共通部分文字列（LCS）の長さを計算する
pub fn longest_common_subsequence(s1: &str, s2: &str) -> usize {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let m = chars1.len();
    let n = chars2.len();
    
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    for i in 1..=m {
        for j in 1..=n {
            if chars1[i - 1] == chars2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    
    dp[m][n]
}

// 編集距離（レーベンシュタイン距離）を計算する
pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let m = chars1.len();
    let n = chars2.len();
    
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    
    for i in 1..=m {
        for j in 1..=n {
            if chars1[i - 1] == chars2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
            }
        }
    }
    
    dp[m][n]
}

// 文字列内の最長回文部分文字列を見つける
pub fn longest_palindromic_substring(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return String::new();
    }
    
    let mut start = 0;
    let mut max_len = 1;
    
    // 奇数長の回文をチェック
    for i in 0..n {
        let mut left = i;
        let mut right = i;
        
        while left < n && right < n && chars[left] == chars[right] {
            let len = right - left + 1;
            if len > max_len {
                max_len = len;
                start = left;
            }
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        }
    }
    
    // 偶数長の回文をチェック
    for i in 0..n - 1 {
        let mut left = i;
        let mut right = i + 1;
        
        while left < n && right < n && chars[left] == chars[right] {
            let len = right - left + 1;
            if len > max_len {
                max_len = len;
                start = left;
            }
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        }
    }
    
    chars[start..start + max_len].iter().collect()
}

// 文字列の部分文字列を全て生成する
pub fn generate_substrings(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut result = Vec::new();
    
    for i in 0..n {
        for j in i + 1..=n {
            result.push(chars[i..j].iter().collect());
        }
    }
    
    result
}

// 文字列の全順列を生成する
pub fn generate_permutations(s: &str) -> Vec<String> {
    let mut chars: Vec<char> = s.chars().collect();
    let mut result = Vec::new();
    
    fn permute(chars: &mut Vec<char>, start: usize, result: &mut Vec<String>) {
        if start == chars.len() {
            result.push(chars.iter().collect());
            return;
        }
        
        for i in start..chars.len() {
            chars.swap(start, i);
            permute(chars, start + 1, result);
            chars.swap(start, i);
        }
    }
    
    permute(&mut chars, 0, &mut result);
    result
}

// 文字列内でパターンを検索する（KMP法）
pub fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let n = text_chars.len();
    let m = pattern_chars.len();
    
    if m == 0 {
        return vec![];
    }
    
    // LPS配列を構築
    let mut lps = vec![0; m];
    let mut len = 0;
    let mut i = 1;
    
    while i < m {
        if pattern_chars[i] == pattern_chars[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    
    // KMP検索
    let mut result = Vec::new();
    let mut i = 0; // text用のインデックス
    let mut j = 0; // pattern用のインデックス
    
    while i < n {
        if pattern_chars[j] == text_chars[i] {
            i += 1;
            j += 1;
        }
        
        if j == m {
            result.push(i - j);
            j = lps[j - 1];
        } else if i < n && pattern_chars[j] != text_chars[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }
    
    result
}

// 文字列を指定した文字で分割する
pub fn split_string(s: &str, delimiter: char) -> Vec<String> {
    s.split(delimiter).map(|s| s.to_string()).collect()
}

// 文字列配列を指定した文字で結合する
pub fn join_strings(strings: &[String], delimiter: &str) -> String {
    strings.join(delimiter)
}

// 文字列の先頭と末尾の空白を削除する
pub fn trim_string(s: &str) -> String {
    s.trim().to_string()
}

// 文字列を大文字に変換する
pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

// 文字列を小文字に変換する
pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

// 文字列内の特定の文字を全て置換する
pub fn replace_char(s: &str, old_char: char, new_char: char) -> String {
    s.chars().map(|c| if c == old_char { new_char } else { c }).collect()
}

// 文字列内の特定の部分文字列を全て置換する
pub fn replace_substring(s: &str, old_str: &str, new_str: &str) -> String {
    s.replace(old_str, new_str)
}

// 文字列が指定したプレフィックスで始まるかチェックする
pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

// 文字列が指定したサフィックスで終わるかチェックする
pub fn ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

// 文字列内の特定の文字の出現回数をカウントする
pub fn count_char(s: &str, target: char) -> usize {
    s.chars().filter(|&c| c == target).count()
}

// 文字列内の特定の部分文字列の出現回数をカウントする
pub fn count_substring(s: &str, target: &str) -> usize {
    if target.is_empty() {
        return 0;
    }
    
    let mut count = 0;
    let mut start = 0;
    
    while let Some(pos) = s[start..].find(target) {
        count += 1;
        start += pos + target.len();
    }
    
    count
}

// 文字列の最長共通プレフィックスを見つける
pub fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let first = &strings[0];
    let mut prefix = String::new();
    
    for (i, ch) in first.chars().enumerate() {
        for s in strings.iter().skip(1) {
            if s.len() <= i || s.chars().nth(i).unwrap() != ch {
                return prefix;
            }
        }
        prefix.push(ch);
    }
    
    prefix
}

// 文字列をランレングス符号化する
pub fn run_length_encode(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return String::new();
    }
    
    let mut result = String::new();
    let mut current_char = chars[0];
    let mut count = 1;
    
    for i in 1..chars.len() {
        if chars[i] == current_char {
            count += 1;
        } else {
            result.push(current_char);
            result.push_str(&count.to_string());
            current_char = chars[i];
            count = 1;
        }
    }
    
    result.push(current_char);
    result.push_str(&count.to_string());
    result
}

// ランレングス符号化された文字列をデコードする
pub fn run_length_decode(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut i = 0;
    
    while i < chars.len() {
        if i + 1 < chars.len() {
            let ch = chars[i];
            let count_str: String = chars[i + 1..].iter().take_while(|&&c| c.is_ascii_digit()).collect();
            if let Ok(count) = count_str.parse::<usize>() {
                result.push_str(&ch.to_string().repeat(count));
                i += 1 + count_str.len();
            } else {
                result.push(chars[i]);
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    
    result
} 