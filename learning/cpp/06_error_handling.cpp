#include <iostream>
#include <string>
#include <stdexcept> // 標準的な例外クラスに必要

// C++入門: 06 - エラー処理

// プログラムは常に正常に動作するとは限りません。不正な入力、ファイルが見つからない、
// ネットワーク接続が切れるなど、様々なエラーが発生する可能性があります。
// C++では、主に「例外処理」という仕組みを使って、これらのエラーに対処します。

// 割り算を行う関数。0での割り算はエラーなので、例外を投げる。
double divide(double numerator, double denominator) {
    // 事前条件のチェック
    if (denominator == 0) {
        // `throw`キーワードで例外を「投げる」
        // `std::runtime_error`は、実行時に発生するエラーを示す標準的な例外クラスです。
        throw std::runtime_error("Error: Division by zero!");
    }
    return numerator / denominator;
}

// 年齢を検証する関数。無効な年齢の場合は例外を投げる。
void validate_age(int age) {
    if (age < 0 || age > 150) {
        // `std::invalid_argument`は、不正な引数が渡されたことを示す例外クラスです。
        throw std::invalid_argument("Error: Age must be between 0 and 150.");
    }
    std::cout << "Valid age entered: " << age << std::endl;
}

int main() {
    // 1. try-catchブロック
    // --------------------
    // 例外が発生する可能性のあるコードを`try`ブロックで囲み、
    // 発生した例外を`catch`ブロックで捕まえます。

    std::cout << "--- 例外処理の基本 --- " << std::endl;
    try {
        // 正常なケース
        double result1 = divide(10.0, 2.0);
        std::cout << "10.0 / 2.0 = " << result1 << std::endl;

        // エラーが発生するケース
        std::cout << "Attempting to divide by zero..." << std::endl;
        double result2 = divide(5.0, 0.0);
        // 上の行で例外がスローされるため、この行は実行されない
        std::cout << "This line will not be executed." << std::endl;

    } catch (const std::runtime_error& e) {
        // `catch`ブロックで、指定した型の例外を捕まえる
        // `e.what()`で、例外オブジェクトに格納されたエラーメッセージを取得できる
        std::cerr << "Caught an exception: " << e.what() << std::endl;
    }

    std::cout << "\nProgram continues after the first try-catch block." << std::endl;

    // 2. 複数のcatchブロック
    // --------------------
    // 異なる型の例外を、それぞれ別の`catch`ブロックで処理できます。

    std::cout << "\n--- 複数の例外処理 --- " << std::endl;
    try {
        // validate_age(-5); // invalid_argumentをスロー
        divide(1.0, 0.0); // runtime_errorをスロー

    } catch (const std::invalid_argument& e) {
        std::cerr << "Caught an invalid argument: " << e.what() << std::endl;
    } catch (const std::runtime_error& e) {
        std::cerr << "Caught a runtime error: " << e.what() << std::endl;
    } catch (...) { // `...` は、あらゆる型の例外をキャッチする（最後の砦）
        std::cerr << "Caught an unknown exception." << std::endl;
    }

    std::cout << "\n--- 例外処理のデモ終了 ---" << std::endl;

    return 0;
}

/*
なぜエラー処理が重要か？
- 堅牢性: 予期せぬエラーでプログラム全体がクラッシュするのを防ぎます。
- 明確さ: エラーが発生した箇所と、それを処理する箇所をコード上で明確に分離できます。
- リソース管理: 例外が発生しても、RAII(Resource Acquisition Is Initialization)というC++の仕組み
  （スマートポインタなどがその例）により、確保したリソース（メモリ、ファイル、ロックなど）を
  安全に解放できます。

練習問題:
1. 文字列を整数に変換する`string_to_int`という関数を書いてみましょう。
   - `std::stoi`という標準関数が使えますが、これは変換できない文字列が渡されると
     `std::invalid_argument`や`std::out_of_range`といった例外をスローします。
   - `try-catch`ブロックを使ってこの例外を捕まえ、成功した場合は整数を、
     失敗した場合はエラーメッセージを表示するプログラムを作成してください。

2. `std::vector`から指定されたインデックスの要素を取得する関数を考えます。
   `vector.at(index)`は、範囲外のインデックスが指定されると`std::out_of_range`例外をスローします。
   この性質を利用して、安全に要素を取得し、範囲外アクセスの場合はエラーメッセージを
   表示するプログラムを作成してください。
*/
