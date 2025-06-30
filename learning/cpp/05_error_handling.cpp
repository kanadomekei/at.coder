// C++ と Zig のエラー処理の比較
#include <iostream>
#include <string>
#include <vector>
#include <stdexcept> // std::runtime_error などの標準例外クラス
#include <optional>  // C++17: 値が存在しない可能性を表す
#include <expected>  // C++23: 正常値またはエラーを返す

// --- Zig との比較用コメント ---
// Zigのエラー処理は、言語に深く統合されています。
// エラーは `error` セットとして定義され、関数は `!` を使ってエラーを
// 返す可能性があることを明示します。
//
// const DivisionError = error {
//     DivisionByZero,
// };
//
// fn divide(numerator: f64, denominator: f64) DivisionError!f64 {
//     if (denominator == 0) {
//         return DivisionError.DivisionByZero;
//     }
//     return numerator / denominator;
// }
//
// pub fn main() !void {
//     // `try` はエラーを呼び出し元に伝播させる
//     const result = try divide(10, 2);
//
//     // `catch` でエラーを処理する
//     const result_or_err = divide(10, 0);
//     const safe_result = result_or_err catch |err| {
//         // エラー処理
//         std.debug.print("Error: {any}\n", .{err});
//         return 0;
//     };
// }
// --------------------------

// --- 1. 例外処理 (try, catch, throw) ---
// 伝統的なC++のエラー処理方法。エラーが発生すると、通常の実行フローを中断し、
// 呼び出しスタックを遡って対応する `catch` ブロックを探す。

// 割り算を行う関数。不正な引数の場合に例外を投げる(throw)
double divide(double numerator, double denominator) {
    if (denominator == 0) {
        // エラー情報をruntime_errorオブジェクトに込めて投げる
        throw std::runtime_error("Division by zero condition!");
    }
    return numerator / denominator;
}

void process_division() {
    try {
        // 例外が発生する可能性のあるコードを try ブロックで囲む
        double result = divide(10.0, 2.0);
        std::cout << "[Exception] 10.0 / 2.0 = " << result << std::endl;

        result = divide(5.0, 0.0); // ここで例外がスローされる
        std::cout << "This line will not be executed." << std::endl;

    } catch (const std::runtime_error& e) {
        // スローされた例外を catch ブロックで捕捉する
        // e.what() でエラーメッセージを取得
        std::cerr << "[Exception] Caught a runtime_error: " << e.what() << std::endl;
    } catch (const std::exception& e) {
        // その他の標準例外を捕捉
        std::cerr << "[Exception] Caught a general exception: " << e.what() << std::endl;
    } catch (...) {
        // 予期しないあらゆる例外を捕捉
        std::cerr << "[Exception] Caught an unknown exception." << std::endl;
    }
}

// --- 2. std::optional (C++17) ---
// 値が存在するか、しないか、の2つの状態を表す。エラーの原因は伝えない。
// "見つからなかった" のような、エラーではない失敗を表すのに適している。
std::optional<int> find_user_id(const std::string& name) {
    if (name == "Alice") {
        return 42; // 値を返す
    }
    return std::nullopt; // 値が存在しないことを示す
}

void process_optional() {
    std::optional<int> alice_id = find_user_id("Alice");
    if (alice_id.has_value()) { // or just `if (alice_id)`
        std::cout << "[Optional] Alice's ID: " << alice_id.value() << std::endl;
    }

    std::optional<int> bob_id = find_user_id("Bob");
    if (!bob_id) {
        std::cout << "[Optional] Bob's ID not found." << std::endl;
    }
    // .value() は空の場合に例外を投げるので注意
    // .value_or(default_value) を使うと安全
    std::cout << "[Optional] Bob's ID (safe): " << bob_id.value_or(-1) << std::endl;
}

// --- 3. std::expected (C++23) ---
// 正常値か、エラー値か、のどちらかを保持する。エラーの理由を伝えられる。
// Zig の `error!T` に最も近い考え方。

enum class ParseError {
    InvalidInput,
    Overflow,
};

std::expected<int, ParseError> parse_integer(const std::string& str) {
    try {
        size_t pos;
        int result = std::stoi(str, &pos);
        // 文字列がすべて数字で構成されているかチェック
        if (pos != str.length()) {
            return std::unexpected(ParseError::InvalidInput);
        }
        return result;
    } catch (const std::out_of_range&) {
        return std::unexpected(ParseError::Overflow);
    } catch (...) {
        return std::unexpected(ParseError::InvalidInput);
    }
}

void process_expected() {
    auto result1 = parse_integer("123");
    if (result1.has_value()) { // or `if (result1)`
        std::cout << "[Expected] Parsed 123 successfully: " << *result1 << std::endl;
    }

    auto result2 = parse_integer("abc");
    if (!result2) {
        // .error() でエラー値を取得
        ParseError err = result2.error();
        if (err == ParseError::InvalidInput) {
            std::cout << "[Expected] Failed to parse 'abc': Invalid Input" << std::endl;
        }
    }
}

int main() {
    std::cout << "=== C++ と Zig のエラー処理の比較 ===" << std::endl;

    std::cout << "\n--- 1. 例外処理 ---" << std::endl;
    process_division();
    // --- Zig との比較 ---
    // C++の例外は、エラーを処理する場所と発生する場所が離れていても良い（大域脱出）。
    // これは時に便利ですが、コードの制御フローを分かりにくくする欠点もあります。
    // Zigは `try` によるエラーの明示的な伝播を強制することで、どこでエラーが
    // 発生しうるかを明確にしています。

    std::cout << "\n--- 2. std::optional (C++17) ---" << std::endl;
    process_optional();
    // --- Zig との比較 ---
    // Zigでは `?T` (optional type) がこれに相当します。
    // `var maybe_value: ?i32 = null;`
    // `if (maybe_value) |value| { ... }` という構文で安全に値を取り出せます。

    std::cout << "\n--- 3. std::expected (C++23) ---" << std::endl;
    process_expected();
    // --- Zig との比較 ---
    // `std::expected<T, E>` は、Zigの `E!T` に思想的に非常に近いです。
    // どちらも、関数のシグネチャを見るだけで、成功時の型と失敗時のエラー型が
    // 何であるかが明確に分かります。これにより、呼び出し側はエラー処理を
    // 忘れにくくなり、より堅牢なコードを書くことができます。

    std::cout << "\n=== エラー処理の学習完了 ===" << std::endl;

    return 0;
}
