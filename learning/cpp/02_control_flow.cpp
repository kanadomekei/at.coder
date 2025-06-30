// C++ と Zig の制御構文の比較
#include <iostream>
#include <vector>
#include <string>

// --- Zig との比較用コメント ---
// Zig の制御構文は、C++と似ている部分もありますが、より表現力が高く、
// 安全性を重視した設計になっています。
//
// pub fn main() !void {
//     // if文: C++と似ていますが、`()` は不要です。
//     const score = 85;
//     if (score >= 90) {
//         // ...
//     } else if (score >= 70) {
//         // ...
//     } else {
//         // ...
//     }
//
//     // switch文: C++より強力で、網羅的(exhaustive)であることが要求されます。
//     const value = 1;
//     switch (value) {
//         1 => std.debug.print("One\n", .{}),
//         2 => std.debug.print("Two\n", .{}),
//         else => std.debug.print("Other\n", .{}), // `else` が必須
//     }
//
//     // for文: イテレータブルなものに対して使います。
//     const items = [_]u8{ 1, 2, 3 };
//     for (items) |item, index| {
//         std.debug.print("items[{}] = {}\n", .{index, item});
//     }
//
//     // while文: C++と似ています。
//     var i: u8 = 0;
//     while (i < 5) : (i += 1) {
//         std.debug.print("{}\n", .{i});
//     }
// }
// --------------------------

int main() {
    std::cout << "=== C++ と Zig の制御構文の比較 ===" << std::endl;

    // --- 1. if-else if-else 文 ---
    std::cout << "\n--- 1. if-else if-else 文 ---" << std::endl;
    int score = 85;
    if (score >= 90) {
        std::cout << "Grade: A" << std::endl;
    } else if (score >= 80) {
        std::cout << "Grade: B" << std::endl;
    } else {
        std::cout << "Grade: C or below" << std::endl;
    }

    // C++17以降: if文の初期化子
    if (int new_score = 75; new_score >= 60) {
        std::cout << "new_score (" << new_score << ") is passing." << std::endl;
    } // new_score のスコープはここまで

    // --- Zig との比較 ---
    // Zigのif文は式(expression)であり、値を返すことができます。
    // `const result = if (x > 10) "big" else "small";`
    // C++では三項演算子 `? :` がこれに似た役割を果たします。
    std::string size = (score > 80) ? "Large" : "Small";
    std::cout << "Score size (ternary operator): " << size << std::endl;

    // --- 2. switch 文 ---
    std::cout << "\n--- 2. switch 文 ---" << std::endl;
    int day = 3;
    switch (day) {
        case 1:
            std::cout << "Monday";
            break; // `break` を忘れると次のcaseに処理が続く (フォールスルー)
        case 2:
            std::cout << "Tuesday";
            break;
        case 3:
        case 4:
            std::cout << "Wednesday or Thursday"; // 複数のcaseをまとめる
            break;
        default:
            std::cout << "Some other day";
            break;
    }
    std::cout << std::endl;

    // C++17以降: switch文の初期化子
    switch (int val = 42; val) {
        case 42: std::cout << "The answer!" << std::endl; break;
        default: std::cout << "Not the answer." << std::endl; break;
    }

    // --- Zig との比較 ---
    // Zigのswitchはフォールスルーせず、各ブランチは独立しています。
    // また、`else` ブランチによって全ての可能性を網羅することがコンパイラによって
    // 強制されるため、`break`の書き忘れのようなバグが起こりません。

    // --- 3. for ループ ---
    std::cout << "\n--- 3. for ループ ---" << std::endl;
    // C-style forループ
    std::cout << "C-style for loop: ";
    for (int i = 0; i < 5; ++i) { // `++i` (前置) が `i++` (後置) より好まれることがある
        std::cout << i << " ";
    }
    std::cout << std::endl;

    // 範囲ベース forループ (C++11以降)
    std::cout << "Range-based for loop: ";
    std::vector<int> numbers = {10, 20, 30, 40, 50};
    for (int num : numbers) {
        std::cout << num << " ";
    }
    std::cout << std::endl;

    // 参照を使った範囲ベースfor (要素の変更が可能)
    std::cout << "Range-based for with reference: ";
    for (int &num : numbers) {
        num += 1; // 各要素に1を加える
    }
    for (const int &num : numbers) { // 読み取り専用の場合は `const&` が効率的
        std::cout << num << " ";
    }
    std::cout << std::endl;

    // --- Zig との比較 ---
    // Zigのforは、C++の範囲ベースforに似ています。
    // `for (items) |item| { ... }`
    // C-styleの `for (int i=0; ...)` に直接対応するものはありませんが、
    // `while`ループやイテレータで同様の処理を実現します。

    // --- 4. while / do-while ループ ---
    std::cout << "\n--- 4. while / do-while ループ ---" << std::endl;
    int countdown = 3;
    std::cout << "while loop: ";
    while (countdown > 0) {
        std::cout << countdown-- << " ";
    }
    std::cout << std::endl;

    int input;
    std::cout << "do-while loop (executes at least once): ";
    do {
        // std::cout << "Enter a number (0 to exit): ";
        // std::cin >> input;
        input = 0; // サンプル実行のため固定値
        std::cout << "You entered: " << input << std::endl;
    } while (input != 0);

    // --- Zig との比較 ---
    // ZigのwhileもC++と似ていますが、`continue`式に値を渡してループの次の
    // イテレーションの変数を更新する、といった高度な機能も持ちます。
    // `while (i < 10) : (i += 1) { ... }` のように、ループの最後に実行される式を書けます。

    // --- 5. break と continue ---
    std::cout << "\n--- 5. break と continue ---" << std::endl;
    std::cout << "Loop with break/continue: ";
    for (int i = 1; i <= 10; ++i) {
        if (i % 2 != 0) {
            continue; // 奇数の場合はこのイテレーションをスキップ
        }
        if (i > 8) {
            break; // 8より大きくなったらループを抜ける
        }
        std::cout << i << " "; // 2 4 6 8 が出力される
    }
    std::cout << std::endl;

    // --- Zig との比較 ---
    // Zigにも `break` と `continue` は存在し、同様に機能します。
    // 加えて、Zigでは `break` に値を渡してループ式全体の結果とすることができます。
    // `const found_item = blk: { for (items) |item| { if (item == target) break :blk item; } else null; };`

    std::cout << "\n=== 制御構文の学習完了 ===" << std::endl;

    return 0;
}