#include <iostream>
#include <vector>
#include <string>
#include <numeric> // std::accumulate に必要
#include <algorithm> // std::sort, std::max_element に必要
#include <map>

// C++入門: 09 - 実践的な例

// これまでに学んだ知識を組み合わせて、より実践的な問題を解いてみましょう。
// ここでは、競技プログラミングでよく見られるような、いくつかの典型的な問題を取り上げます。

// --- 例題1: 合計値の計算 ---
// N個の整数が与えられるので、その合計値を計算してください。
void solve_sum() {
    std::cout << "--- 例題1: 合計値 ---" << std::endl;
    std::vector<int> numbers = {10, 20, 30, 40, 50};
    
    // 方法1: forループを使う
    long long sum1 = 0; // 合計値が大きくなる可能性があるのでlong longを使う
    for (int num : numbers) {
        sum1 += num;
    }
    std::cout << "forループでの合計: " << sum1 << std::endl;

    // 方法2: std::accumulate を使う (numericヘッダ)
    // より簡潔で意図が明確
    long long sum2 = std::accumulate(numbers.begin(), numbers.end(), 0LL);
    // 0LLは long long 型の 0 を意味する
    std::cout << "std::accumulateでの合計: " << sum2 << std::endl;
}

// --- 例題2: 最大値とそのインデックス ---
// N個の整数が与えられるので、その中の最大値と、それが最初に出現した位置（0-indexed）を求めてください。
void solve_max_value() {
    std::cout << "\n--- 例題2: 最大値とそのインデックス ---" << std::endl;
    std::vector<int> scores = {60, 95, 80, 95, 70};

    // 方法1: forループで探す
    int max_val = -1;
    int max_idx = -1;
    for (int i = 0; i < scores.size(); ++i) {
        if (scores[i] > max_val) {
            max_val = scores[i];
            max_idx = i;
        }
    }
    std::cout << "forループでの最大値: " << max_val << ", インデックス: " << max_idx << std::endl;

    // 方法2: STLアルゴリズムを使う (algorithmヘッダ)
    auto max_it = std::max_element(scores.begin(), scores.end());
    int max_val2 = *max_it;
    int max_idx2 = std::distance(scores.begin(), max_it);
    std::cout << "STLでの最大値: " << max_val2 << ", インデックス: " << max_idx2 << std::endl;
}

// --- 例題3: カードのカウンティング ---
// N枚のカードがあり、それぞれに文字列が書かれています。
// 各文字列が何回出現したかを数え上げてください。
void solve_counting() {
    std::cout << "\n--- 例題3: カードのカウンティング ---" << std::endl;
    std::vector<std::string> cards = {"apple", "orange", "apple", "banana", "orange", "apple"};

    // std::map を使ってカウントするのが定石
    std::map<std::string, int> counts;
    for (const std::string& card : cards) {
        counts[card]++;
    }

    // 結果の表示
    for (const auto& pair : counts) {
        std::cout << pair.first << ": " << pair.second << "回" << std::endl;
    }
}

// --- 例題4: 条件を満たすペアの探索 ---
// N個の整数からなる数列があります。和がちょうど100になる2つの異なる要素のペアは存在しますか？
void solve_pair_sum() {
    std::cout << "\n--- 例題4: 和が100になるペア ---" << std::endl;
    std::vector<int> a = {10, 80, 30, 90, 40, 70};
    bool found = false;

    // 方法1: 二重ループ (Nが小さい場合に有効)
    for (int i = 0; i < a.size(); ++i) {
        for (int j = i + 1; j < a.size(); ++j) {
            if (a[i] + a[j] == 100) {
                found = true;
                std::cout << "ペア発見 (二重ループ): (" << a[i] << ", " << a[j] << ")" << std::endl;
                break; // 内側ループを抜ける
            }
        }
        if (found) {
            break; // 外側ループも抜ける
        }
    }

    if (!found) {
        std::cout << "ペアは見つかりませんでした。" << std::endl;
    }
    
    // (参考) より効率的な方法: ソートやハッシュマップ(std::unordered_set)を使う方法もある
}


int main() {
    // 各例題の関数を呼び出す
    solve_sum();
    solve_max_value();
    solve_counting();
    solve_pair_sum();

    return 0;
}

/*
ここから先へ:
- AtCoderなどの競技プログラミングサイトの問題に挑戦してみましょう。
  - A問題(Beginner Contest)は、基本的な入出力と簡単な計算が中心です。
  - B問題は、ループや条件分岐、配列操作などが問われます。
- `cin`での標準入力の受け取り方を学びましょう。
  (例: `int N; std::cin >> N;`)
- 計算量(オーダー)を意識することを始めましょう。Nが大きくなると、二重ループ(O(N^2))では
  間に合わなくなることがあります。
*/
