
#include <iostream>
#include <string>
#include <vector>

// C++入門: 01 - 変数と型

// C++は静的型付け言語です。つまり、変数の型はコンパイル時に決定されます。
// このファイルでは、C++の基本的な変数の宣言方法と、主要なデータ型について学びます。

int main() {
    // 1. 基本的なデータ型
    // --------------------

    // `int`: 整数を格納します (例: -1, 0, 123)
    int age = 30;
    std::cout << "年齢 (int): " << age << std::endl;

    // `double`: 倍精度浮動小数点数を格納します (例: 3.14, -0.001)
    // `float`もありますが、`double`の方が一般的で精度が高いです。
    double pi = 3.14159;
    std::cout << "円周率 (double): " << pi << std::endl;

    // `char`: 1文字を格納します (例: 'a', 'Z', '?')
    char initial = 'K';
    std::cout << "イニシャル (char): " << initial << std::endl;

    // `bool`: 真偽値を格納します (true または false)
    bool is_active = true;
    std::cout << "アクティブ (bool): " << is_active << std::endl; // 出力は1 (true) または 0 (false)

    // `std::string`: 文字列を格納します。
    // <string>ヘッダーのインクルードが必要です。
    std::string name = "Taro Yamada";
    std::cout << "名前 (string): " << name << std::endl;


    // 2. 型推論 (auto)
    // --------------------
    // `auto`キーワードを使うと、コンパイラが初期値から型を自動で推論してくれます。
    auto country = "Japan"; // コンパイラは const char* と推論し、std::stringに変換可能
    auto year = 2025;       // コンパイラは int と推論
    auto gravity = 9.8;     // コンパイラは double と推論

    std::cout << "国 (auto -> string): " << country << std::endl;
    std::cout << "年 (auto -> int): " << year << std::endl;
    std::cout << "重力 (auto -> double): " << gravity << std::endl;


    // 3. 定数 (const)
    // --------------------
    // `const`キーワードを付けると、変数の値を変更できなくなります。
    const int SECONDS_PER_MINUTE = 60;
    std::cout << "1分あたりの秒数 (const): " << SECONDS_PER_MINUTE << std::endl;
    // SECONDS_PER_MINUTE = 61; // この行はコンパイルエラーになります


    // 4. 配列とベクター
    // --------------------

    // C-style配列: 固定長の配列
    int scores[3] = {85, 92, 78};
    std::cout << "スコア[0] (C-style array): " << scores[0] << std::endl;

    // `std::vector`: 可変長の動的配列。より安全で柔軟なため、C++ではこちらが推奨されます。
    // <vector>ヘッダーのインクルードが必要です。
    std::vector<int> numbers;
    numbers.push_back(10); // 要素を追加
    numbers.push_back(20);
    numbers.push_back(30);
    std::cout << "ベクター[1] (std::vector): " << numbers[1] << std::endl;


    // 5. 型のサイズ (sizeof)
    // --------------------
    // `sizeof`演算子で、各データ型がメモリ上で占めるサイズ（バイト単位）を確認できます。
    // サイズは環境（OSやCPUアーキテクチャ）によって異なる場合があります。
    std::cout << "sizeof(int): " << sizeof(int) << " bytes" << std::endl;
    std::cout << "sizeof(double): " << sizeof(double) << " bytes" << std::endl;
    std::cout << "sizeof(std::string): " << sizeof(std::string) << " bytes" << std::endl;

    return 0;
}

/*
練習問題:
1. あなたの好きな数字を`int`型の変数に格納し、表示してみましょう。
2. あなたのフルネームを`std::string`型の定数として宣言し、表示してみましょう。
3. 3つの都市名を`std::vector<std::string>`に格納し、2番目の都市名を表示してみましょう。
*/
