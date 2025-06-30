# C++ 競技プログラミングガイド

C++は競技プログラミングで最も人気の高い言語の一つです。この資料では、AtCoderで必要なC++の文法と競技プログラミング特有のテクニックを解説します。

## 📚 学習ファイル一覧

1. [基本文法と変数](01_basics_variables.cpp) - 変数、データ型、基本演算
2. [制御構文](02_control_flow.cpp) - if文、ループ、switch文
3. [関数とスコープ](03_functions.cpp) - 関数定義、引数、再帰
4. [配列とベクター](04_arrays_vectors.cpp) - 配列、vector、多次元配列
5. [文字列処理](05_strings.cpp) - string操作、文字列アルゴリズム
6. [STLコンテナ](06_stl_containers.cpp) - map, set, queue, stack等
7. [アルゴリズム関数](07_algorithms.cpp) - sort, binary_search等
8. [数学とビット演算](08_math_bits.cpp) - 数学関数、ビット操作
9. [実践的なテクニック](09_competitive_techniques.cpp) - 競技プログラミング特有のテクニック

## 🚀 AtCoder での基本テンプレート

```cpp
#include <iostream>
#include <vector>
#include <algorithm>
#include <string>
#include <map>
#include <set>
#include <queue>
#include <stack>
#include <cmath>
#include <climits>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    
    // ここに解答コードを書く
    
    return 0;
}
```

## 📖 基本概念

### 高速化設定

```cpp
ios::sync_with_stdio(false);  // C言語のstdioとの同期を無効化
cin.tie(nullptr);             // cinとcoutの結び付けを解除
```

### よく使う型と定数

```cpp
typedef long long ll;
const int INF = 1e9;
const ll LINF = 1e18;
const double EPS = 1e-9;
```

### 入出力の基本

```cpp
// 基本的な入力
int n;
cin >> n;

// 配列の入力
vector<int> a(n);
for(int i = 0; i < n; i++) {
    cin >> a[i];
}

// 文字列の入力
string s;
cin >> s;                    // 空白区切り
getline(cin, s);            // 行全体
```

## 🎯 学習のポイント

### 重要度の高い概念

1. **STLコンテナ**: vector, map, set, queue, stack
2. **アルゴリズム関数**: sort, binary_search, lower_bound
3. **文字列処理**: 文字列操作と解析
4. **数学関数**: 最大公約数、素数判定、組み合わせ
5. **ビット演算**: フラグ管理、高速化

### 競技プログラミング特有のテクニック

1. **マクロの活用**: よく使うコードの短縮
2. **高速入出力**: 実行時間制限への対策
3. **メモ化**: 動的プログラミングの実装
4. **貪欲法**: 最適化問題のアプローチ
5. **全探索**: 小さな制約での解法

## 📊 計算量の目安

| データサイズ | 計算量 | アルゴリズム例 |
|---|---|---|
| N ≤ 10 | O(N!) | 全順列探索 |
| N ≤ 20 | O(2^N) | bit全探索 |
| N ≤ 500 | O(N^3) | ワーシャルフロイド法 |
| N ≤ 5000 | O(N^2) | バブルソート |
| N ≤ 10^5 | O(N log N) | マージソート |
| N ≤ 10^6 | O(N) | 線形探索 |

## 🔧 デバッグテクニック

```cpp
// デバッグマクロ
#ifdef DEBUG
#define debug(x) cerr << #x << " = " << x << endl;
#else
#define debug(x)
#endif

// 配列の出力
template<typename T>
void print_vector(const vector<T>& v) {
    for(int i = 0; i < v.size(); i++) {
        cout << v[i] << (i == v.size()-1 ? "\n" : " ");
    }
}
```

## 📝 実行方法

### ファイル実行

```bash
# 各サンプルファイルのコンパイルと実行
g++ -std=c++17 -o main 01_basics_variables.cpp && ./main
g++ -std=c++17 -o main 02_control_flow.cpp && ./main
g++ -std=c++17 -o main 03_functions.cpp && ./main
# ... 他のファイルも同様
```

### デバッグビルド

```bash
g++ -std=c++17 -g -DDEBUG -Wall -Wextra -o main file.cpp
```

### 最適化ビルド

```bash
g++ -std=c++17 -O2 -o main file.cpp
```

## 🏆 競技プログラミングでよく出る問題パターン

1. **実装問題**: 問題文通りに実装
2. **全探索**: すべての可能性を調べる
3. **貪欲法**: 局所的に最適な選択
4. **動的プログラミング**: 部分問題の解を記録
5. **グラフ問題**: DFS, BFS, 最短経路
6. **数学**: 整数論、組み合わせ、確率

## 📚 次のステップ

1. 各学習ファイルを順番に実行・理解
2. AtCoderのA～C問題で実践
3. よく出るアルゴリズムパターンの習得
4. 過去問の解法パターン分析
5. コンテスト参加で実戦経験を積む

C++は高速でライブラリが充実しており、競技プログラミングに最適です。基本文法を押さえ、STLを活用して効率的な解答を目指しましょう！