#include <iostream>
#include <string>
#include <vector>

// C++入門: 08 - テンプレート

// テンプレートは、C++におけるジェネリックプログラミングの根幹をなす機能です。
// これにより、特定の型に依存しない、汎用的な関数やクラスを作成することができます。
// STLの`vector`や`map`なども、このテンプレートを使って実装されています。

// 1. 関数テンプレート
// --------------------
// `template <typename T>` という宣言を関数の前に追加することで、
// その関数を任意の型 `T` で動作させることができます。

template <typename T>
T get_max(T a, T b) {
    return (a > b) ? a : b;
}

// 複数の型引数を取ることも可能
template <typename T, typename U>
void print_pair(T first, U second) {
    std::cout << "(" << first << ", " << second << ")" << std::endl;
}

// 2. クラステンプレート
// --------------------
// クラスも同様にテンプレート化できます。
// これにより、任意の型を格納できるデータ構造などを作成できます。

template <typename T>
class Pair {
private:
    T first_;
    T second_;

public:
    Pair(T first, T second) : first_(first), second_(second) {}

    T get_first() const {
        return first_;
    }

    T get_second() const {
        return second_;
    }

    void set_first(T value) {
        first_ = value;
    }
};


int main() {
    // --- 関数テンプレートの利用 ---
    std::cout << "--- 関数テンプレート ---" << std::endl;

    // int型でget_maxを呼び出し
    int max_int = get_max(10, 20);
    std::cout << "Max of 10, 20 is " << max_int << std::endl;

    // double型でget_maxを呼び出し
    double max_double = get_max(3.14, 2.71);
    std::cout << "Max of 3.14, 2.71 is " << max_double << std::endl;

    // string型でget_maxを呼び出し
    std::string s1 = "apple";
    std::string s2 = "orange";
    std::cout << "Max of \"apple\", \"orange\" is \"" << get_max(s1, s2) << "\"" << std::endl;

    // 複数の型引数を持つテンプレートの呼び出し
    print_pair("Score", 95);
    print_pair(1, 2.5);


    // --- クラステンプレートの利用 ---
    std::cout << "\n--- クラステンプレート ---" << std::endl;

    // int型を格納するPairオブジェクト
    Pair<int> int_pair(5, 10);
    std::cout << "Int Pair: (" << int_pair.get_first() << ", " << int_pair.get_second() << ")" << std::endl;

    // std::string型を格納するPairオブジェクト
    Pair<std::string> string_pair("Hello", "World");
    std::cout << "String Pair: (" << string_pair.get_first() << ", " << string_pair.get_second() << ")" << std::endl;

    // `std::vector`もクラステンプレートの一例
    // `std::vector<int>` は int型を格納するベクター
    // `std::vector<std::string>` は string型を格納するベクター
    std::vector<int> numbers = {1, 2, 3};

    return 0;
}

/*
なぜテンプレートが重要か？
- コードの再利用性: 同じロジック（例えば、最大値を求める、ソートするなど）を
  異なるデータ型のために何度も書く必要がなくなります。
- 型安全性: コンパイル時に型チェックが行われるため、実行時エラーのリスクを減らしつつ、
  柔軟なコードを書くことができます。
- 効率性: テンプレートはコンパイル時に特定の型に対するコードを生成するため、
  実行時のパフォーマンス低下がほとんどありません。

練習問題:
1. 任意の型の`std::vector`を受け取り、その全ての要素を画面に表示する
   `print_vector`という関数テンプレートを書いてみましょう。

2. 3つの引数を受け取り、その中の最小値を返す`get_min3`という関数テンプレートを書いてみましょう。

3. 任意の型の値を3つ保持する`Triple`というクラステンプレートを書いてみましょう。
   コンストラクタと、それぞれの値を取得する`get_first()`, `get_second()`, `get_third()`
   といったメソッドを持たせてください。
*/
