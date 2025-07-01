
#include <iostream>
#include <string>

// C++入門: 03 - 関数

// 関数は、特定のタスクを実行するコードのブロックです。
// コードを再利用し、整理し、管理しやすくするために不可欠です。

// 1. 関数の定義
// --------------------
// 戻り値の型 関数名(引数の型 引数名, ...) { ... }

// 引数も戻り値もない、最もシンプルな関数
void say_hello() {
    std::cout << "こんにちは、世界！" << std::endl;
}

// 引数を受け取り、戻り値を返さない関数
void greet(const std::string& name) {
    std::cout << "こんにちは、" << name << "さん！" << std::endl;
}

// 2つの整数を受け取り、その合計を返す関数
int add(int a, int b) {
    return a + b;
}

// 2. 関数のオーバーロード
// --------------------
// C++では、同じ関数名で引数の型や数が異なる関数を複数定義できます。
// これを「関数のオーバーロード」と呼びます。

// double型の引数を2つ受け取るadd関数
double add(double a, double b) {
    return a + b;
}

// 3. デフォルト引数
// --------------------
// 関数の引数には、呼び出し時に省略された場合に適用されるデフォルト値を設定できます。
void show_message(const std::string& message, const std::string& prefix = "[INFO]") {
    std::cout << prefix << " " << message << std::endl;
}

int main() {
    // --- 関数の呼び出し ---

    // say_hello関数の呼び出し
    say_hello();

    // greet関数の呼び出し
    std::string user_name = "Keita";
    greet(user_name);

    // add関数の呼び出しと結果の利用
    int sum_int = add(5, 3);
    std::cout << "5 + 3 = " << sum_int << std::endl;

    // オーバーロードされたadd関数の呼び出し
    double sum_double = add(2.5, 3.7);
    std::cout << "2.5 + 3.7 = " << sum_double << std::endl;

    // デフォルト引数を持つ関数の呼び出し
    show_message("処理が完了しました。"); // prefixを省略
    show_message("致命的なエラーが発生しました。", "[ERROR]"); // prefixを指定


    // 4. main関数
    // --------------------
    // `main`関数は、C++プログラムが実行されるときに最初に呼び出される特別な関数です。
    // `return 0;` はプログラムが正常に終了したことをOSに伝えます。

    return 0;
}

/*
練習問題:
1. 2つの整数を引数として受け取り、そのうち大きい方を返す`max`という名前の関数を書いてみましょう。
2. 1つの`std::string`と1つの`int`を引数に取り、その文字列をintの回数だけ表示する`repeat_string`という関数を書いてみましょう。
3. 三角形の底辺と高さを引数として受け取り、面積を計算して返す`triangle_area`という関数を書いてみましょう。(面積 = 底辺 * 高さ / 2)
*/
