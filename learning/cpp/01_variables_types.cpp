// C++ と Zig の変数と型の比較
#include <iostream>
#include <string>
#include <vector>
#include <typeinfo> // typeid を使うために必要
#include <cstdint>  // 固定幅整数型 (int64_tなど) のために追加

// C++11以降で推奨される型エイリアス
using ll = long long;

// C++03 以前の型エイリアス (C互換)
typedef long long old_ll;

// --- Zig との比較用コメント ---
// (省略)
// --------------------------

int main() {
    std::cout << "=== C++ と Zig の変数と型の比較 ===" << std::endl;

    // --- 1. 変数宣言 ---
    std::cout << "\n--- 1. 変数宣言 ---" << std::endl;
    int age = 30;
    double height = 175.8;
    char initial = 'K';
    bool isProgrammer = true;

    std::cout << "Age: " << age << std::endl;
    std::cout << "Height: " << height << " cm" << std::endl;
    std::cout << "Initial: " << initial << std::endl;
    std::cout << "Is a programmer? " << (isProgrammer ? "Yes" : "No") << std::endl;

    // --- 2. 型推論 (auto) ---
    std::cout << "\n--- 2. 型推論 (auto) ---" << std::endl;
    auto city = "Tokyo";
    auto population = 14000000;
    auto pi = 3.14159;

    std::cout << "City: " << city << " (type: " << typeid(city).name() << ")" << std::endl;
    std::cout << "Population: " << population << " (type: " << typeid(population).name() << ")" << std::endl;
    std::cout << "Pi: " << pi << " (type: " << typeid(pi).name() << ")" << std::endl;

    // --- 3. 定数 (const / constexpr) ---
    std::cout << "\n--- 3. 定数 (const / constexpr) ---" << std::endl;
    const int MAX_USERS = 100;
    constexpr double EULER_NUMBER = 2.71828;

    std::cout << "Max Users (const): " << MAX_USERS << std::endl;
    std::cout << "Euler's Number (constexpr): " << EULER_NUMBER << std::endl;

    // --- 4. 基本的なデータ型 ---
    std::cout << "\n--- 4. 基本的なデータ型 ---" << std::endl;
    short s = 10;
    int i = 100;
    long l = 100000;
    long long ll_var = 10000000000; // 変数名を ll_var に変更

    float f = 3.14f;
    double d = 3.14159;
    std::string str = "Hello, C++!";

    std::cout << "long long variable: " << ll_var << std::endl;
    std::cout << "string: " << str << std::endl;
    
    // 固定幅整数型
    int64_t fixed_width_int = 987654321098765432LL;
    std::cout << "int64_t: " << fixed_width_int << std::endl;

    // --- 5. 型エイリアス (using / typedef) ---
    std::cout << "\n--- 5. 型エイリアス (using / typedef) ---" << std::endl;
    ll big_number = 1234567890123LL; // ここで `ll` 型エイリアスを使用
    old_ll another_big_number = 456LL;

    std::cout << "using (ll): " << big_number << std::endl;
    std::cout << "typedef (old_ll): " << another_big_number << std::endl;

    std::cout << "\n=== 変数と型の学習完了 ===" << std::endl;

    return 0;
}