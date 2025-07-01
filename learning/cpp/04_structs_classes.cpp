
#include <iostream>
#include <string>

// C++入門: 04 - 構造体とクラス

// 構造体(struct)とクラス(class)は、関連するデータと関数を一つにまとめるための仕組みです。
// これにより、より複雑なデータをモデル化し、コードを整理することができます。
// これらはオブジェクト指向プログラミング(OOP)の基本的な構成要素です。

// 1. 構造体 (struct)
// --------------------
// 複数の変数をグループ化するための単純なデータコンテナです。
// デフォルトでは、全てのメンバーが公開(public)されます。

struct Point {
    double x;
    double y;
};

void print_point(const Point& p) {
    std::cout << "Point(x: " << p.x << ", y: " << p.y << ")" << std::endl;
}

// 2. クラス (class)
// --------------------
// 構造体と似ていますが、より高機能で、カプセル化を重視します。
// デフォルトでは、全てのメンバーが非公開(private)になります。
// public, private, protected といったアクセス修飾子を使って、外部からのアクセスを制御します。

class Player {
private:
    // --- メンバー変数 (データを保持) ---
    std::string name_;
    int level_;
    int hp_;

public:
    // --- コンストラクタ (オブジェクト生成時に呼ばれる特別な関数) ---
    Player(const std::string& name, int initial_level) {
        name_ = name;
        level_ = initial_level;
        hp_ = 100; // 初期HPは100で固定
        std::cout << name_ << "がゲームに参加しました！" << std::endl;
    }

    // --- メンバー関数 (メソッド) ---

    // プレイヤーの情報を表示する
    void display_status() const { // `const`はメンバー変数を変更しないことを示す
        std::cout << "--- ステータス ---" << std::endl;
        std::cout << "名前: " << name_ << std::endl;
        std::cout << "レベル: " << level_ << std::endl;
        std::cout << "HP: " << hp_ << std::endl;
        std::cout << "------------------" << std::endl;
    }

    // レベルアップ処理
    void level_up() {
        level_++;
        hp_ += 10; // レベルアップでHPが10回復
        std::cout << name_ << "はレベル" << level_ << "になった！" << std::endl;
    }

    // ダメージを受ける処理
    void take_damage(int damage) {
        hp_ -= damage;
        if (hp_ < 0) {
            hp_ = 0;
        }
        std::cout << name_ << "は" << damage << "のダメージを受けた！ 残りHP: " << hp_ << std::endl;
    }
};

int main() {
    // --- 構造体の利用 ---
    Point p1;
    p1.x = 10.5;
    p1.y = 20.0;
    print_point(p1);

    // --- クラスの利用 ---
    // Playerクラスのインスタンス（オブジェクト）を作成
    Player player1("Hero", 1);

    // publicなメンバー関数を呼び出す
    player1.display_status();
    player1.level_up();
    player1.take_damage(30);
    player1.display_status();

    // player1.hp_ = 999; // これはコンパイルエラー！ hp_はprivateなので直接アクセスできない

    return 0;
}

/*
練習問題:
1. `Book`という名前の構造体を定義してみましょう。
   メンバーとして `title` (std::string), `author` (std::string), `price` (int) を持たせます。
   `Book`オブジェクトを作成し、各メンバーに値を設定して表示するプログラムを書いてみましょう。

2. `Car`という名前のクラスを定義してみましょう。
   - privateメンバー: `gasoline_` (int), `speed_` (int)
   - publicメンバー:
     - コンストラクタ: `Car()` (ガソリンを50で初期化)
     - `accelerate()`: speedを10上げ、gasolineを5消費する
     - `brake()`: speedを10下げる (0未満にはならない)
     - `refuel(int amount)`: gasolineを増やす
     - `show_status()`: 現在のspeedとgasolineを表示する
   `Car`オブジェクトを作成し、いくつかのメソッドを呼び出して動作を確認してみましょう。
*/
