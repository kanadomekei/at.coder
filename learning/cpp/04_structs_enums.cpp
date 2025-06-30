// C++ と Zig の構造体と列挙体の比較
#include <iostream>
#include <string>
#include <vector>

// --- Zig との比較用コメント ---
// Zig の `struct` は、データフィールドとメソッドの両方を持つことができます。
// C++の `struct` や `class` に近いです。
//
// const Point = struct {
//     x: f32,
//     y: f32,
//
//     fn print(self: Point) void {
//         std.debug.print("Point(x={}, y={})\n", .{self.x, self.y});
//     }
// };
//
// Zig の `enum` は、単なる整数値のエイリアスではなく、タグ付きユニオン(tagged union)としても
// 機能し、各バリアントがデータを持つことができます。非常に強力です。
//
// const Color = enum {
//     Red,
//     Green,
//     Blue,
//     Rgb: struct { r: u8, g: u8, b: u8 },
// };
// --------------------------

// --- 1. 構造体 (struct) ---
// 関連するデータを一つにまとめるための型
struct Player {
    // publicなメンバ変数 (デフォルト)
    std::string name;
    int level;
    double health;

    // メンバ関数 (メソッド)
    void printInfo() {
        std::cout << "Player: " << name << " | Level: " << level << " | Health: " << health << "%" << std::endl;
    }

    // `this` ポインタ: メンバ関数内で、そのオブジェクト自身を指すポインタ
    void takeDamage(double damage) {
        this->health -= damage;
        if (this->health < 0) {
            this->health = 0;
        }
    }
};

// C++では、`struct` と `class` はほぼ同じものです。
// 唯一の違いは、デフォルトのアクセス指定子です。
// - struct: デフォルトで `public`
// - class:  デフォルトで `private`
class Point {
    // privateなメンバ変数 (デフォルト)
    double x;
    double y;

public:
    // コンストラクタ: オブジェクトが生成されるときに呼ばれる特殊な関数
    Point(double x_val, double y_val) {
        x = x_val;
        y = y_val;
    }

    // メンバ初期化子リストを使った、よりモダンで効率的なコンストラクタ
    // Point(double x_val, double y_val) : x(x_val), y(y_val) {}

    void print() {
        std::cout << "Point(x=" << x << ", y=" << y << ")" << std::endl;
    }
};

// --- 2. 列挙体 (enum) ---
// a) 古いC-style enum
// スコープがなく、整数型に暗黙的に変換されるため、問題を起こしやすい
enum OldColor {
    RED, // 0
    GREEN, // 1
    BLUE // 2
};

// b) スコープ付き列挙体 (enum class) - C++11以降で強く推奨
// スコープを持ち、型安全性が高い
enum class HttpStatus : int { // 基底となる型を指定できる
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500
};

// enum class を使う関数
std::string getStatusMessage(HttpStatus status) {
    switch (status) {
        case HttpStatus::Ok:
            return "OK";
        case HttpStatus::NotFound:
            return "Not Found";
        case HttpStatus::InternalServerError:
            return "Internal Server Error";
        default:
            return "Unknown Status";
    }
}

int main() {
    std::cout << "=== C++ と Zig の構造体と列挙体の比較 ===" << std::endl;

    std::cout << "\n--- 1. 構造体の使い方 ---" << std::endl;
    Player player1;
    player1.name = "Aragorn";
    player1.level = 15;
    player1.health = 100.0;

    player1.printInfo();
    player1.takeDamage(25.5);
    player1.printInfo();

    std::cout << "\n--- class の使い方 ---" << std::endl;
    Point p1(10.5, 20.3);
    p1.print();
    // p1.x = 5.0; // コンパイルエラー: x は private

    // --- Zig との比較 ---
    // C++の `struct`/`class` は、Zigの `struct` に似ています。
    // Zigでは、`public` や `private` のようなアクセス制御は言語機能としてはありません。
    // APIの設計によって公開するものを制御するのが一般的です。
    // また、C++のコンストラクタは特別な構文ですが、Zigでは単なる慣習的な関数です
    // (例: `fn init(...) Self { ... }`)。

    std::cout << "\n--- 2. 列挙体の使い方 ---" << std::endl;
    OldColor old_c = GREEN; // スコープがない
    if (old_c == 1) { // 整数と簡単に比較できてしまう
        std::cout << "OldColor GREEN is equal to integer 1." << std::endl;
    }

    HttpStatus status = HttpStatus::NotFound;
    std::cout << "Status: " << getStatusMessage(status) << std::endl;
    // if (status == 404) { // コンパイルエラー: enum class は整数と直接比較できない
    // } 
    // 比較するには明示的なキャストが必要
    if (static_cast<int>(status) == 404) {
        std::cout << "Status code is indeed 404." << std::endl;
    }

    // --- Zig との比較 ---
    // C++の `enum class` は、Zigの単純なenum (`const Color = enum { Red, Green };`)
    // に似ていますが、Zigのenumはさらに強力です。
    // Zigのenumは、各バリアントが値を持つことができる「タグ付きユニオン」として機能します。
    // これにより、C++の `std::variant` や `std::optional` のような機能を、より簡潔な
    // 構文で実現できます。

    std::cout << "\n=== 構造体と列挙体の学習完了 ===" << std::endl;

    return 0;
}
