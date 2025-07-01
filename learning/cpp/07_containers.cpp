#include <iostream>
#include <vector>
#include <string>
#include <map>
#include <set>
#include <algorithm> // sortなどのアルゴリズムに必要

// C++入門: 07 - STLコンテナ

// C++の標準テンプレートライブラリ(STL)は、プログラミングで頻繁に使われる
// データ構造（コンテナ）やアルゴリズムを提供します。
// これらを使いこなすことで、効率的で信頼性の高いコードを素早く書くことができます。

int main() {
    // 1. `std::vector` - 動的配列
    // --------------------
    // 最もよく使われるコンテナ。要素を末尾に効率的に追加できます。
    std::cout << "--- std::vector ---" << std::endl;
    std::vector<int> vec = {10, 30, 20};

    // 要素の追加
    vec.push_back(40);

    // 要素へのアクセス
    std::cout << "最初の要素: " << vec[0] << std::endl;
    std::cout << "最後の要素: " << vec.back() << std::endl;

    // サイズ
    std::cout << "ベクターのサイズ: " << vec.size() << std::endl;

    // 全要素のループ
    for (int val : vec) {
        std::cout << val << " ";
    }
    std::cout << std::endl;

    // ソート (algorithmヘッダが必要)
    std::sort(vec.begin(), vec.end());
    std::cout << "ソート後: ";
    for (int val : vec) {
        std::cout << val << " ";
    }
    std::cout << std::endl;


    // 2. `std::string` - 文字列
    // --------------------
    // 文字のシーケンスを扱います。内部的には`std::vector<char>`のように動作します。
    std::cout << "\n--- std::string ---" << std::endl;
    std::string s = "Hello";
    s += ", World!"; // 文字列の連結
    std::cout << s << std::endl;
    std::cout << "文字列の長さ: " << s.length() << std::endl;
    std::cout << "部分文字列: " << s.substr(7, 5) << std::endl; // 7番目から5文字


    // 3. `std::map` - 連想配列 (キーと値のペア)
    // --------------------
    // キーを使って値に高速にアクセスできます。キーは自動的にソートされます。
    // (例: 電話帳、単語の頻度カウント)
    std::cout << "\n--- std::map ---" << std::endl;
    std::map<std::string, int> age_map;

    // 要素の追加
    age_map["Alice"] = 25;
    age_map["Bob"] = 30;
    age_map.insert({"Charlie", 22});

    // 要素へのアクセス
    std::cout << "Bobの年齢: " << age_map["Bob"] << std::endl;

    // キーの存在チェック
    if (age_map.count("David")) {
        std::cout << "Davidは存在します。" << std::endl;
    } else {
        std::cout << "Davidは存在しません。" << std::endl;
    }

    // 全要素のループ
    for (const auto& pair : age_map) {
        std::cout << pair.first << " is " << pair.second << " years old." << std::endl;
    }


    // 4. `std::set` - 集合
    // --------------------
    // 重複しない要素を自動的にソートして格納します。
    // ある要素が集合に含まれているかを高速に判定できます。
    std::cout << "\n--- std::set ---" << std::endl;
    std::set<int> unique_numbers;

    unique_numbers.insert(10);
    unique_numbers.insert(50);
    unique_numbers.insert(20);
    unique_numbers.insert(10); // 重複した要素は無視される

    // 要素の存在チェック
    if (unique_numbers.count(20)) {
        std::cout << "20は集合に含まれています。" << std::endl;
    }

    // 全要素のループ (自動的にソートされている)
    std::cout << "集合の要素: ";
    for (int num : unique_numbers) {
        std::cout << num << " ";
    }
    std::cout << std::endl;

    return 0;
}

/*
練習問題:
1. ユーザーに5つの整数を入力してもらい、それらを`std::vector`に格納した後、
   合計値と平均値を計算して表示するプログラムを書いてみましょう。

2. ある文章(string)に含まれる各単語の出現回数をカウントするプログラムを`std::map<std::string, int>`を使って書いてみましょう。
   (簡単のため、単語はスペースで区切られているものとします)

3. 2つの`std::vector<int>`を受け取り、両方のベクターに含まれる要素だけを
   `std::set<int>`に入れて返す関数を書いてみましょう（積集合）。
*/
