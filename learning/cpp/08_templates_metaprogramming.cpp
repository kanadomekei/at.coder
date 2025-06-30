// C++ と Zig のテンプレートとコンパイル時処理の比較
#include <iostream>
#include <string>
#include <vector>
#include <type_traits> // is_integral, enable_if_t など

// --- Zig との比較用コメント ---
// Zigのジェネリックプログラミングとコンパイル時処理は `comptime` という
// 一つの統一された概念に基づいています。
// `comptime` ブロック内のコードは、コンパイル時に実行されます。
//
// // ジェネリックな関数: T はコンパイル時に決定される型
// fn add(a: anytype, b: anytype) @TypeOf(a) {
//     return a + b;
// }
//
// // コンパイル時計算
// const factorial = comptime blk: {
//     var res: u64 = 1;
//     for (1..=10) |i| {
//         res *= i;
//     }
//     break :blk res;
// };
//
// // `comptime` は非常に強力で、型を返す関数を書いたり、コンパイル時に
// // 条件分岐を行ったり、複雑なデータ構造を構築したりできます。
// // C++のテンプレートメタプログラミング(TMP)が達成しようとすることを、
// // より直接的で分かりやすい構文で実現します。
// --------------------------

// --- 1. 関数テンプレート ---
// 型を引数として受け取る関数。様々な型に対して同じ操作を行うコードを記述できる。
// `typename T` または `class T` でテンプレートパラメータを宣言する。
template <typename T>
T findMax(T a, T b) {
    return (a > b) ? a : b;
}

// --- 2. クラステンプレート ---
// 型を引数として受け取るクラス。様々な型を格納するコンテナなどを実装するのに使われる。
// (例: std::vector<T>, std::array<T, Size>)
template <typename T, size_t Size>
class FixedArray {
private:
    T m_data[Size];
public:
    T& operator[](size_t index) { return m_data[index]; }
    const T& operator[](size_t index) const { return m_data[index]; }
    size_t size() const { return Size; }
};

// --- 3. テンプレートの特殊化 (Specialization) ---
// 特定の型に対して、テンプレートの実装を上書きする。
// 例えば、`const char*` (C-style文字列) はポインタ比較ではなく文字列比較が必要。
template <>
const char* findMax<const char*>(const char* a, const char* b) {
    return (std::strcmp(a, b) > 0) ? a : b;
}

// --- 4. SFINAE (Substitution Failure Is Not An Error) と `if constexpr` ---
// C++のテンプレートメタプログラミング(TMP)のテクニックの一つ。
// 「テンプレートの置き換えに失敗しても、それはコンパイルエラーではない」という原則。
// これを利用して、特定の条件を満たす型でのみ関数を有効にする、といった制御が可能。

// a) C++11/14 style (SFINAE with std::enable_if)
// Tが整数型の場合のみ、この関数が有効になる
template <typename T>
typename std::enable_if<std::is_integral<T>::value, void>::type
print_if_integer(T value) {
    std::cout << "[SFINAE] Integer value: " << value << std::endl;
}

// Tが整数型でない場合、上の関数は候補から外れ、こちらが呼ばれる
template <typename T>
typename std::enable_if<!std::is_integral<T>::value, void>::type
print_if_integer(T value) {
    std::cout << "[SFINAE] Not an integer." << std::endl;
}

// b) C++17 style (`if constexpr`)
// コンパイル時に条件分岐を行う。より直感的で読みやすい。
template <typename T>
void print_if_integer_cpp17(T value) {
    if constexpr (std::is_integral_v<T>) { // _v は ::value のショートカット
        std::cout << "[if constexpr] Integer value: " << value << std::endl;
    } else {
        std::cout << "[if constexpr] Not an integer." << std::endl;
    }
}

// --- 5. コンパイル時計算 (constexpr) ---
// `constexpr` を使うと、関数や変数をコンパイル時に評価できる。
// Zigの `comptime` に最も近い考え方。
constexpr long long factorial(int n) {
    return n <= 1 ? 1 : n * factorial(n - 1);
}

int main() {
    std::cout << "=== C++ と Zig のテンプレートとコンパイル時処理の比較 ===" << std::endl;

    std::cout << "\n--- 1. 関数テンプレートの使用 ---" << std::endl;
    std::cout << "Max of 10, 20: " << findMax(10, 20) << std::endl;
    std::cout << "Max of 3.14, 2.71: " << findMax(3.14, 2.71) << std::endl;
    std::cout << "Max of std::string: " << findMax(std::string("apple"), std::string("orange")) << std::endl;

    std::cout << "\n--- 2. クラステンプレートの使用 ---" << std::endl;
    FixedArray<int, 5> intArray;
    for (size_t i = 0; i < intArray.size(); ++i) {
        intArray[i] = i * 10;
    }
    std::cout << "intArray[3] = " << intArray[3] << std::endl;

    std::cout << "\n--- 3. テンプレートの特殊化の使用 ---" << std::endl;
    std::cout << "Max of \"hello\", \"world\": " << findMax("hello", "world") << std::endl;

    std::cout << "\n--- 4. コンパイル時条件分岐 ---" << std::endl;
    print_if_integer(123);
    print_if_integer(4.56);
    print_if_integer_cpp17(789);
    print_if_integer_cpp17(std::string("text"));

    // --- Zig との比較 (SFINAE / if constexpr) ---
    // C++のSFINAEは非常に複雑で、長年テンプレートメタプログラミングの難解さの
    // 原因とされてきました。`if constexpr` はこれを大幅に改善しました。
    // Zigの `comptime` は、最初から `if constexpr` のように直感的に
    // コンパイル時の条件分岐を書けるため、学習コストが低く、コードも読みやすいです。
    // `if (@TypeOf(value) == i32) { ... } else { ... }` のように書けます。

    std::cout << "\n--- 5. コンパイル時計算 (constexpr) ---" << std::endl;
    constexpr long long fact10 = factorial(10);
    std::cout << "Factorial of 10 (calculated at compile time): " << fact10 << std::endl;

    // 配列のサイズとしてコンパイル時計算の結果を使用する
    std::array<int, factorial(5)> compile_time_sized_array;
    std::cout << "Size of array determined by factorial(5): " << compile_time_sized_array.size() << std::endl;

    // --- Zig との比較 (constexpr) ---
    // C++の `constexpr` は、Zigの `comptime` に相当する機能です。
    // どちらも、コンパイラがコードを実行してその結果をプログラムに埋め込むことを
    // 可能にします。これにより、実行時のコストを削減したり、より柔軟なコード生成を
    // 行ったりすることができます。
    // Zigの `comptime` は、変数宣言、if文、ブロックなど、より多くの場所で
    // 利用でき、言語全体にわたって一貫した構文で提供されている点が特徴です。

    std::cout << "\n=== テンプレートとコンパイル時処理の学習完了 ===" << std::endl;

    return 0;
}