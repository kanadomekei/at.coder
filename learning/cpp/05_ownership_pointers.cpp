#include <iostream>
#include <string>
#include <memory> // スマートポインタに必要

// C++入門: 05 - 所有権とポインタ

// C++では、メモリ管理をプログラマが意識する必要があります。
// このファイルでは、メモリ上の特定のアドレスを指し示す「ポインタ」と、
// 安全なメモリ管理を助ける「スマートポインタ」について学びます。

// 値を書き換える関数（ポインタ渡し）
void add_five_pointer(int* ptr) {
    if (ptr) { // ポインタがヌルでないことを確認
        *ptr += 5; // `*` (間接参照演算子)でポインタが指す先の値にアクセス
    }
}

// 値を書き換える関数（参照渡し）
void add_ten_reference(int& ref) {
    ref += 10; // 参照はそのまま変数として扱える
}

int main() {
    // 1. ポインタ (Raw Pointers)
    // --------------------
    // ポインタは、変数のメモリアドレスを格納する変数です。
    // `*` を型の後につけて宣言し、`&` (アドレス演算子)で変数のアドレスを取得します。

    int value = 20;
    int* ptr_to_value = &value; // valueのアドレスをptr_to_valueに格納

    std::cout << "--- ポインタの基本 ---" << std::endl;
    std::cout << "value の値: " << value << std::endl;
    std::cout << "value のメモリアドレス: " << ptr_to_value << std::endl;
    std::cout << "ptr_to_value が指す先の値: " << *ptr_to_value << std::endl;

    // ポインタを介して元の変数の値を変更する
    *ptr_to_value = 25;
    std::cout << "ポインタ経由で変更後の value の値: " << value << std::endl;

    // 関数にポインタを渡す
    add_five_pointer(ptr_to_value);
    std::cout << "add_five_pointer適用後の value の値: " << value << std::endl;

    // 動的メモリ確保 (new / delete)
    // プログラム実行中にメモリを確保します。使い終わったら必ず`delete`で解放する必要があります。
    // 解放を忘れると「メモリリーク」の原因になります。
    int* dynamic_int = new int(100);
    std::cout << "\n動的に確保した値: " << *dynamic_int << std::endl;
    *dynamic_int = 150;
    std::cout << "変更後の値: " << *dynamic_int << std::endl;
    delete dynamic_int; // メモリ解放！
    dynamic_int = nullptr; // 解放後はヌルポインタを代入するのが安全


    // 2. 参照 (References)
    // --------------------
    // 参照は、既存の変数に対する「別名」です。ポインタと似ていますが、より安全で直感的に使えます。
    // - 宣言時に必ず初期化が必要
    // - 一度設定したら、他の変数を指すようには変更できない
    // - ヌルになることはない

    int original = 50;
    int& ref_to_original = original; // originalの別名としてref_to_originalを宣言

    std::cout << "\n--- 参照の基本 ---" << std::endl;
    std::cout << "original の値: " << original << std::endl;
    ref_to_original = 55; // 参照を変更すると、元の変数も変更される
    std::cout << "参照経由で変更後の original の値: " << original << std::endl;

    // 関数に参照を渡す
    add_ten_reference(original);
    std::cout << "add_ten_reference適用後の original の値: " << original << std::endl;


    // 3. スマートポインタ (Smart Pointers)
    // --------------------
    // `new`と`delete`の手動管理は、メモリリークやダングリングポインタ（無効なメモリを指すポインタ）
    // といったバグの温床です。C++11以降では、これらを自動化するスマートポインタが推奨されます。

    // a) `std::unique_ptr`: 所有権が唯一のポインタ
    //    コピーできず、スコープを抜けると自動的にメモリを解放します。
    { // スコープを作成
        std::unique_ptr<int> u_ptr = std::make_unique<int>(200);
        std::cout << "\n--- unique_ptr ---" << std::endl;
        std::cout << "unique_ptrが指す値: " << *u_ptr << std::endl;
        // スコープを抜けるときに自動でメモリが解放される
    }
    std::cout << "unique_ptrのスコープを抜けました。" << std::endl;

    // b) `std::shared_ptr`: 複数のポインタが所有権を共有
    //    参照カウンタを持っており、全てのshared_ptrが破棄されたときにメモリを解放します。
    std::shared_ptr<std::string> sh_ptr1;
    { // スコープを作成
        std::shared_ptr<std::string> sh_ptr2 = std::make_shared<std::string>("Hello, Shared Pointer!");
        sh_ptr1 = sh_ptr2; // 所有権を共有 (参照カウンタが2になる)
        std::cout << "\n--- shared_ptr ---" << std::endl;
        std::cout << "sh_ptr2が指す値: " << *sh_ptr2 << std::endl;
        std::cout << "参照カウント: " << sh_ptr1.use_count() << std::endl;
    } // sh_ptr2がスコープを抜ける (参照カウンタが1になる)
    std::cout << "sh_ptr2のスコープを抜けました。" << std::endl;
    std::cout << "参照カウント: " << sh_ptr1.use_count() << std::endl;
    std::cout << "sh_ptr1が指す値: " << *sh_ptr1 << std::endl;

    return 0;
}

/*
まとめ:
- 生ポインタ(`new`/`delete`)は強力ですが、メモリリークなどのリスクが伴います。
- 関数に値を渡して変更させたい場合は、ポインタよりも参照の方が安全で簡単です。
- 動的に確保したメモリの管理には、可能な限りスマートポインタ(`std::unique_ptr`, `std::shared_ptr`)を使いましょう。
  これにより、コードが安全になり、メモリ管理の心配が大幅に減ります。
*/
