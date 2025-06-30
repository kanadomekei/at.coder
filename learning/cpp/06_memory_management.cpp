// C++ と Zig のメモリ管理の比較
#include <iostream>
#include <string>
#include <vector>
#include <memory> // スマートポインタ (unique_ptr, shared_ptr) のために必要

// --- Zig との比較用コメント ---
// Zigは手動メモリ管理を採用しており、C言語に近い考え方です。
// ただし、より安全でモダンな仕組みが提供されています。
//
// const std = @import("std");
//
// // メモリ確保は `Allocator` を介して行われる
// fn doSomething(allocator: std.mem.Allocator) !void {
//     // メモリ確保
//     const data = try allocator.alloc(u8, 1024);
//
//     // `defer` を使うことで、スコープの終わりに必ず実行される処理を記述できる
//     // これにより、リソースの解放忘れを防ぐ (C++のRAIIに似た効果)
//     defer allocator.free(data);
//
//     // ... メモリを使った処理 ...
// }
//
// Zigでは、どこでメモリが確保・解放されるかがコード上非常に明確です。
// `try` を使ったエラー処理と組み合わせることで、リソースリークを防ぎやすい
// 設計になっています。
// --------------------------

// Player構造体の定義 (04_structs_enums.cpp から拝借)
struct Player {
    std::string name;
    int level;
    double health;

    void printInfo() {
        std::cout << "Player: " << name << " | Level: " << level << " | Health: " << health << "%" << std::endl;
    }

    void takeDamage(double damage) {
        this->health -= damage;
        if (this->health < 0) {
            this->health = 0;
        }
    }
};

// --- 1. RAII (Resource Acquisition Is Initialization) --- 
// C++のメモリ管理とリソース管理の根幹をなす非常に重要な概念。
// オブジェクトの生存期間(lifetime)とそのオブジェクトが管理するリソースの生存期間を
// 一致させる設計パターン。
// オブジェクトが生成されるとき(コンストラクタ)にリソースを取得し、
// オブジェクトが破棄されるとき(デストラクタ)にリソースを解放する。

class FileHandler {
private:
    FILE* pFile;
    std::string filename;

public:
    // コンストラクタ: ファイルを開く (リソース取得)
    FileHandler(const std::string& fname) : filename(fname) {
        std::cout << "[RAII] Opening file: " << filename << std::endl;
        pFile = fopen(filename.c_str(), "w+");
        if (!pFile) {
            throw std::runtime_error("Failed to open file.");
        }
    }

    // デストラクタ: スコープを抜けるときに自動で呼ばれ、ファイルを閉じる (リソース解放)
    ~FileHandler() {
        std::cout << "[RAII] Closing file: " << filename << std::endl;
        if (pFile) {
            fclose(pFile);
        }
    }

    void write(const std::string& text) {
        if (pFile) {
            fputs(text.c_str(), pFile);
        }
    }
    // コピーを禁止して、リソースの二重解放を防ぐ
    FileHandler(const FileHandler&) = delete;
    FileHandler& operator=(const FileHandler&) = delete;
};

void process_raii() {
    try {
        FileHandler f("test.txt"); // スコープに入るとファイルが開かれる
        f.write("Hello, RAII!");
        // この関数の終わり ( `}` ) で f のデストラクタが自動的に呼ばれ、ファイルが閉じられる
    } catch (const std::exception& e) {
        std::cerr << e.what() << std::endl;
    }
}

// --- 2. スマートポインタ --- 
// RAIIをポインタで実現したもの。動的に確保したメモリを自動で解放してくれる。
// モダンC++では、`new` と `delete` を直接使うことは稀で、ほとんどの場合
// スマートポインタで代替される。

// a) std::unique_ptr: 所有権(ownership)を一つに限定するポインタ
// コピーはできず、ムーブのみ可能。最も軽量で、デフォルトで使うべきスマートポインタ。
void process_unique_ptr() {
    // `std::make_unique` を使うのが推奨される (C++14以降)
    std::unique_ptr<Player> p1 = std::make_unique<Player>();
    p1->name = "Legolas";
    p1->level = 20;
    p1->health = 100.0;
    p1->printInfo();

    // std::unique_ptr<Player> p2 = p1; // コンパイルエラー: コピーできない
    std::unique_ptr<Player> p3 = std::move(p1); // OK: 所有権を p3 に移動
    std::cout << "p1 is now null: " << (p1 ? "false" : "true") << std::endl;
    p3->printInfo();
    // この関数の終わりで p3 がスコープを抜けると、Playerオブジェクトが自動でdeleteされる
}

// b) std::shared_ptr: 複数のポインタが所有権を共有するポインタ
// 参照カウンタを持っており、最後のポインタが破棄されたときにオブジェクトを解放する。
// 循環参照に注意が必要。
void process_shared_ptr() {
    std::shared_ptr<Player> sp1;
    {
        // `std::make_shared` を使うのが推奨される
        sp1 = std::make_shared<Player>();
        sp1->name = "Gimli";
        sp1->level = 18;
        sp1->health = 150.0;

        std::shared_ptr<Player> sp2 = sp1; // OK: コピー。参照カウンタが2になる
        std::cout << "[Shared] Use count: " << sp1.use_count() << std::endl;
        sp2->takeDamage(50);
    } // sp2がスコープを抜ける。参照カウンタは1になる

    std::cout << "[Shared] Use count after sp2 scope: " << sp1.use_count() << std::endl;
    sp1->printInfo();
    // この関数の終わりで sp1 がスコープを抜けると、参照カウンタが0になり、
    // Playerオブジェクトが自動でdeleteされる
}

int main() {
    std::cout << "=== C++ と Zig のメモリ管理の比較 ===" << std::endl;

    std::cout << "\n--- 1. RAIIによるリソース管理 ---" << std::endl;
    process_raii();
    // --- Zig との比較 ---
    // C++のRAIIは、オブジェクトのデストラクタがスコープを抜ける際に自動で呼ばれる
    // ことを利用しています。これは言語レベルの保証です。
    // Zigの `defer` はこれに似た目的を果たしますが、より手続き的です。
    // `defer` はエラーで関数を抜けるときにも実行が保証されるため、RAIIと
    // 同様にリソースリークを防ぐ強力なツールです。

    std::cout << "\n--- 2. スマートポインタによるメモリ管理 ---" << std::endl;
    std::cout << "\n  a) std::unique_ptr" << std::endl;
    process_unique_ptr();

    std::cout << "\n  b) std::shared_ptr" << std::endl;
    process_shared_ptr();

    // --- Zig との比較 ---
    // Zigにはスマートポインタに直接対応する機能はありません。
    // メモリの所有権は、プログラマが `Allocator` を通じて明示的に管理します。
    // このアプローチは、C++のスマートポインタのような自動化された安全性よりも、
    // 「どこで何が起きているか」という制御の透明性を重視します。
    // ライブラリレベルでスマートポインタのような仕組みを実装することは可能です。

    std::cout << "\n=== メモリ管理の学習完了 ===" << std::endl;

    return 0;
}
