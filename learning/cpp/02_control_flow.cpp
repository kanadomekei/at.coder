#include <iostream>
#include <vector>

// C++入門: 02 - 制御フロー

// プログラムの流れをコントロールするための構文を学びます。
// 条件分岐 (if, else if, else) や繰り返し (for, while) は、あらゆるプログラムの基本です。

int main() {
    // 1. if, else if, else文 (条件分岐)
    // -------------------------------------
    int temperature = 25;

    std::cout << "--- if文の例 --- " << std::endl;
    if (temperature >= 30) {
        std::cout << "暑いですね！" << std::endl;
    } else if (temperature >= 20) {
        std::cout << "快適な気温です。" << std::endl;
    } else {
        std::cout << "少し肌寒いかもしれません。" << std::endl;
    }

    // 2. switch文 (多岐分岐)
    // -------------------------------------
    // 特定の変数の値に応じて、処理を分岐させます。
    // `case`の最後には`break`を忘れないようにしましょう。
    int day_of_week = 3; // 1:月曜, 2:火曜, ...

    std::cout << "\n--- switch文の例 --- " << std::endl;
    switch (day_of_week) {
        case 1:
            std::cout << "月曜日です。" << std::endl;
            break;
        case 2:
            std::cout << "火曜日です。" << std::endl;
            break;
        case 3:
            std::cout << "水曜日です。" << std::endl;
            break;
        case 4:
            std::cout << "木曜日です。" << std::endl;
            break;
        case 5:
            std::cout << "金曜日です。" << std::endl;
            break;
        case 6:
        case 7:
            std::cout << "週末です！" << std::endl;
            break;
        default:
            std::cout << "無効な曜日です。" << std::endl;
            break;
    }

    // 3. forループ (繰り返し)
    // -------------------------------------
    // 指定した回数だけ処理を繰り返します。
    std::cout << "\n--- forループの例 --- " << std::endl;
    for (int i = 0; i < 5; ++i) {
        std::cout << "現在のカウント: " << i << std::endl;
    }

    // 4. 範囲ベースのforループ (Range-based for loop)
    // -------------------------------------
    // 配列やベクターなどのコンテナの全要素に対して処理を行います。
    // こちらの方が簡潔で安全です。
    std::vector<std::string> fruits = {"Apple", "Banana", "Cherry"};
    std::cout << "\n--- 範囲ベースforループの例 --- " << std::endl;
    for (const std::string& fruit : fruits) {
        std::cout << "果物: " << fruit << std::endl;
    }

    // 5. whileループ (条件付き繰り返し)
    // -------------------------------------
    // 指定した条件がtrueである間、処理を繰り返します。
    int countdown = 3;
    std::cout << "\n--- whileループの例 --- " << std::endl;
    while (countdown > 0) {
        std::cout << countdown << "..." << std::endl;
        countdown--; // カウントを減らす
    }
    std::cout << "発射！" << std::endl;

    // 6. do-whileループ
    // -------------------------------------
    // 最低1回は実行されるwhileループです。条件判定がループの後に行われます。
    int number;
    std::cout << "\n--- do-whileループの例 --- " << std::endl;
    do {
        std::cout << "正の数を入力してください: ";
        std::cin >> number;
    } while (number <= 0);
    std::cout << "入力された数: " << number << std::endl;

    return 0;
}

/*
練習問題:
1. 1から10までの整数のうち、偶数だけを表示するforループを書いてみましょう。
   ヒント: `%` (剰余) 演算子を使います (例: `i % 2 == 0`)
2. ユーザーに好きな果物の名前をいくつか入力してもらい、"end"と入力されたらループを終了するプログラムをwhileループを使って書いてみましょう。
3. 1から100までの合計を計算して表示するプログラムを書いてみましょう。
*/
