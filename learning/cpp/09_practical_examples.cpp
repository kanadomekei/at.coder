// C++ と Zig の実践的なコード例の比較
#include <iostream>
#include <vector>
#include <string>
#include <numeric>   // std::accumulate, std::iota
#include <algorithm> // std::sort, std::max_element
#include <map>       // std::map (キーと値のペアを格納する連想配列)

// --- Zig との比較用コメント ---
// Zigの標準ライブラリ `std` には、C++のSTL(Standard Template Library)に
// 相当する、様々なデータ構造やアルゴリズムが含まれています。
//
// - `std.ArrayList(T)`: C++の `std::vector` に相当
// - `std.AutoHashMap(K, V)`: C++の `std::unordered_map` に相当
// - `std.sort.sort(T, slice, context, lessThan)`: C++の `std::sort` に相当
//
// Zigでは、ジェネリックなデータ構造やアルゴリズムは、`comptime` を活用して
// 非常に柔軟に実装されています。また、アロケータを明示的に渡す設計に
// なっているため、メモリ管理の制御がしやすいのが特徴です。
// --------------------------

// --- 1. ジェネリックな Print 関数 (テンプレート活用) ---
// あらゆるコンテナ(vector, arrayなど)の中身を綺麗に出力する関数
template <typename T>
void printContainer(const T& container, const std::string& name = "Container") {
    std::cout << name << " (size: " << container.size() << "): { ";
    // イテレータを使ってコンテナの要素を走査
    for (auto it = container.begin(); it != container.end(); ++it) {
        std::cout << *it << (std::next(it) != container.end() ? ", " : "");
    }
    std::cout << " }" << std::endl;
}

// --- 2. 構造体とソート (演算子オーバーロード) ---
struct Team {
    std::string name;
    int score;

    // `std::sort` などで使えるように、`<` 演算子をオーバーロードする
    bool operator<(const Team& other) const {
        // スコアが高い順にソートしたいので、比較を逆にする
        if (score != other.score) {
            return score > other.score;
        }
        // スコアが同じなら名前の辞書順
        return name < other.name;
    }
};

// --- 3. 頻度カウンター (map活用) ---
// 文字列中の各文字の出現回数を数える
void countCharacterFrequency() {
    std::string text = "hello world, this is a test string.";
    std::map<char, int> freq_map;

    for (char c : text) {
        // a-z の小文字のみカウント
        if (c >= 'a' && c <= 'z') {
            freq_map[c]++; // map[key]++ で簡単にインクリメントできる
        }
    }

    std::cout << "\nCharacter Frequency Map:" << std::endl;
    for (const auto& pair : freq_map) {
        std::cout << "  '" << pair.first << "': " << pair.second << std::endl;
    }
}

int main() {
    std::cout << "=== C++ と Zig の実践的なコード例の比較 ===" << std::endl;

    // --- 1. STLアルゴリズムの活用 ---
    std::cout << "\n--- 1. STLアルゴリズムの活用 ---" << std::endl;
    std::vector<int> numbers = {5, 2, 8, 1, 9, 4, 4};
    printContainer(numbers, "Original vector");

    // a) ソート
    std::sort(numbers.begin(), numbers.end());
    printContainer(numbers, "Sorted vector  ");

    // b) 合計値
    long long sum = std::accumulate(numbers.begin(), numbers.end(), 0LL);
    std::cout << "Sum of elements: " << sum << std::endl;

    // c) 最大値とそのイテレータ
    auto max_it = std::max_element(numbers.begin(), numbers.end());
    if (max_it != numbers.end()) {
        std::cout << "Max element: " << *max_it 
                  << " at index: " << std::distance(numbers.begin(), max_it) << std::endl;
    }

    // --- 2. カスタム構造体のソート ---
    std::cout << "\n--- 2. カスタム構造体のソート ---" << std::endl;
    std::vector<Team> teams = {
        {"Dragons", 80},
        {"Tigers", 95},
        {"Swallows", 80},
        {"Giants", 90}
    };
    
    std::sort(teams.begin(), teams.end()); // Team::operator< が使われる

    std::cout << "Sorted Teams (by score desc, then name asc):" << std::endl;
    for (const auto& team : teams) {
        std::cout << "  - " << team.name << " (" << team.score << ")" << std::endl;
    }
    // --- Zig との比較 ---
    // Zigでカスタムソートを行う場合、比較関数を `std.sort.sort` に渡します。
    // C++の演算子オーバーロードは便利ですが、コードの挙動が暗黙的になる側面も
    // あります。Zigは、どのようなルールでソートするかが呼び出し側で明確に
    // なる、より明示的なアプローチを好みます。

    // --- 3. 頻度カウンター ---
    countCharacterFrequency();
    // --- Zig との比較 ---
    // Zigでは `std.AutoHashMap` を使って同様の機能を実現します。
    // `const count = try map.getOrPut(char, 0);` のようなAPIを使い、
    // キーが存在しない場合の初期値を設定しつつ値を取得・更新します。
    // C++の `map[key]++` は非常に簡潔ですが、キーが存在しない場合に
    // デフォルトコンストラクタで値を生成するという暗黙の動作を伴います。

    std::cout << "\n=== 実践的なコード例の学習完了 ===" << std::endl;

    return 0;
}