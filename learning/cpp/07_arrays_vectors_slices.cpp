// C++ と Zig の配列、ベクター、スライスの比較
#include <iostream>
#include <vector>
#include <array>   // C++11: 固定長配列のラッパー
#include <span>    // C++20: 所有権を持たない配列ビュー

// --- Zig との比較用コメント ---
// Zigの配列とスライスは、言語のコア機能として明確に区別されています。
//
// // [N]T: 固定長配列 (サイズは型の一部)
// const fixed_array: [5]u8 = .{ 1, 2, 3, 4, 5 };
//
// // []T: スライス (ポインタと長さを持つ)
// var slice: []u8 = fixed_array[0..3]; // 配列からスライスを作成
//
// // スライスは、別の配列やスライスの一部を指す「ビュー」です。
// // 所有権は持ちません。
//
// // 可変長配列は、`std.ArrayList(T)` を使って実現します。
// // これはC++の `std::vector` に非常に近いです。
// const list = std.ArrayList(u8).init(allocator);
// defer list.deinit();
// try list.append(10);
// --------------------------

// スライス(ビュー)を引数に取る関数 (C++20 style)
// この関数は、C-style配列、std::array, std::vector のいずれも受け取ることができる
void printSpan(std::span<const int> data) {
    std::cout << "[Span] pointer: " << data.data() << ", size: " << data.size() << std::endl;
    std::cout << "[Span] content: ";
    for (int val : data) {
        std::cout << val << " ";
    }
    std::cout << std::endl;
}

int main() {
    std::cout << "=== C++ と Zig の配列、ベクター、スライスの比較 ===" << std::endl;

    // --- 1. C-style 配列 ---
    // C言語から引き継がれた最も基本的な配列。固定長。
    // サイズ情報が失われやすく、扱いにくい点が多い。
    std::cout << "\n--- 1. C-style 配列 ---" << std::endl;
    int c_array[5] = {1, 2, 3, 4, 5};
    std::cout << "Size of c_array in main: " << sizeof(c_array) << " bytes" << std::endl;
    // printSpan(c_array); // C-style配列からspanへの暗黙変換

    // --- 2. std::array (C++11) ---
    // C-style配列をラップした、より安全で便利な固定長配列コンテナ。
    // サイズ情報を常に保持し、STLのアルゴリズムと連携しやすい。
    std::cout << "\n--- 2. std::array ---" << std::endl;
    std::array<int, 5> std_array = {10, 20, 30, 40, 50};
    std::cout << "std::array size: " << std_array.size() << std::endl;
    std::cout << "Element at index 2: " << std_array.at(2) << std::endl; // 範囲チェック付きアクセス
    // std::cout << std_array.at(10) << std::endl; //範囲外アクセス。例外(out_of_range)をスローする

    // --- Zig との比較 ---
    // `std::array<T, N>` は、Zigの固定長配列 `[N]T` に最も近い存在です。
    // どちらもサイズがコンパイル時に決定され、型情報の一部となります。

    // --- 3. std::vector ---
    // 動的にサイズが変更できる可変長配列。最もよく使われるコンテナの一つ。
    // メモリはヒープ領域に確保される。
    std::cout << "\n--- 3. std::vector ---" << std::endl;
    std::vector<int> vec;
    std::cout << "Initial vector size: " << vec.size() << ", capacity: " << vec.capacity() << std::endl;
    vec.push_back(100); // 末尾に要素を追加
    vec.push_back(200);
    vec.push_back(300);
    std::cout << "Vector after push_back: ";
    for (int val : vec) {
        std::cout << val << " ";
    }
    std::cout << std::endl;
    std::cout << "Vector size: " << vec.size() << ", capacity: " << vec.capacity() << std::endl;
    vec.pop_back(); // 末尾の要素を削除

    // --- Zig との比較 ---
    // `std::vector` は、Zigの `std.ArrayList` に相当します。
    // どちらも内部でメモリの確保・再確保を行い、動的な配列を実現します。
    // C++のvectorはデストラクタで自動的にメモリを解放しますが、ZigのArrayListは
    // `deinit()` を `defer` と共に呼び出して明示的に解放する必要があります。

    // --- 4. std::span (C++20) ---
    // 配列状のデータへの「ビュー」。ポインタとサイズを保持するが、データそのものの
    // 所有権は持たない。関数の引数として非常に有用。
    std::cout << "\n--- 4. std::span (C++20) ---" << std::endl;
    std::cout << "Passing different array types to a function accepting std::span:" << std::endl;

    std::cout << "\n  a) from C-style array:" << std::endl;
    printSpan(c_array);

    std::cout << "\n  b) from std::array:" << std::endl;
    printSpan(std_array);

    std::cout << "\n  c) from std::vector:" << std::endl;
    printSpan(vec);

    std::cout << "\n  d) from a sub-section of a vector:" << std::endl;
    std::vector<int> long_vec = {0, 10, 20, 30, 40, 50};
    // ベクターの2番目から3つの要素を指すスパンを作成
    std::span<const int> sub_span(long_vec.begin() + 2, 3);
    printSpan(sub_span);

    // --- Zig との比較 ---
    // `std::span` は、Zigのスライス `[]T` のコンセプトと全く同じです。
    // どちらも連続したメモリ領域への参照（ポインタと長さ）であり、所有権を持ちません。
    // これにより、異なる種類のコンテナ（固定長配列、動的配列など）を統一的に
    // 扱う関数を、データのコピーなしに安全に記述できます。
    // C++に `span` が導入されたことで、Zigが当初から持っていた強力で安全な
    // スライスの仕組みに近づいたと言えます。

    std::cout << "\n=== 配列、ベクター、スライスの学習完了 ===" << std::endl;

    return 0;
}