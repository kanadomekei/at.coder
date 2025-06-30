# AtCoder 競技プログラミング環境

このリポジトリは、[AtCoder](https://atcoder.jp/) の競技プログラミングに [Go](https://go.dev/) と [Rust](https://www.rust-lang.org/) で取り組むための開発環境です。

[atcoder-cli](https://github.com/Tatamo/atcoder-cli) と [Taskfile](https://taskfile.dev/) を利用して、コンテストの準備からテスト、提出までを効率的に行うことを目的としています。

## 特徴

- **簡単な環境構築**: `acc` と `task` コマンドで、新しいコンテストのプロジェクトを素早くセットアップできます。
- **Rust向けテンプレート**: Rust用のテンプレートは、便利なクレート (`proconio`, `itertools`) を含んだCargoプロジェクトを自動で生成します。
- **効率的なワークフロー**: `task` コマンドを使って、コーディング、テスト、提出のサイクルをスムーズに行えます。

## 必要なツール

- [Go](https://go.dev/)
- [Rust](https://www.rust-lang.org/)
- [Node.js](https://nodejs.org/) (atcoder-cli のため)
- [Python](https://www.python.org/) (online-judge-tools のため)
- [Taskfile](https://taskfile.dev/installation/)
- [atcoder-cli](https://github.com/Tatamo/atcoder-cli)
- [online-judge-tools](https://github.com/online-judge-tools/oj)

## 精進の方法 (基本的なワークフロー)

競技プログラミング（精進）を進めるための基本的な流れは以下の通りです。

### 1. 新しいコンテストの開始

コンテストが始まったら、まず最初にプロジェクトのルートディレクトリで以下のコマンドを実行します。

```bash
# task new -- <コンテストID>
task new -- abc123
```

これにより、`<コンテストID>` (例: `abc123`) の名前でディレクトリが作成されます。
その中には、各問題 (`a`, `b`, `c`, ...) のサブディレクトリが作られます。

さらに、各問題ディレクトリの中には `go/` と `rust/` の2つのディレクトリが作成され、それぞれの言語用のプロジェクトがセットアップされます。

### 2. 問題を解く

解答したい問題と言語のディレクトリに移動します。

```bash
# cd <コンテストID>/<問題ID>/<言語>
cd abc123/a/rust  # Rustで解く場合
# cd abc123/a/go    # Goで解く場合
```

`go/main.go` または `rust/src/main.rs` をエディタで開き、コーディングを開始します。

### 3. 解答をテストする

コードが書けたら、その**言語のディレクトリ内**で以下のコマンドを実行して、サンプルケースをテストします。

```bash
task test
```

`online-judge-tools` が一つ上の階層の `tests` ディレクトリ内のサンプルケースをすべて使って、コードの正しさを検証します。

### 4. 解答を提出する

すべてのサンプルケースで正解したら、同じく**言語のディレクトリ内**で以下のコマンドを実行して、AtCoderに解答を提出します。

```bash
task submit
```

`atcoder-cli` が自動的にその言語の解答ファイルを提出します。

## Taskfile の使い方

このプロジェクトでは、繰り返し行う操作を `Taskfile.yml` にまとめています。

- `task new -- <contest_id>`
  - 新しいコンテストのディレクトリと、各問題のプロジェクトを生成します。
  - プロジェクトのルートディレクトリで実行してください。
  - (例: `task new -- agc049`)

- `task test`
  - 現在いる**言語のディレクトリ**のコードを、サンプルケースでテストします。
  - 問題の言語ディレクトリ (例: `abc123/a/rust`) 内で実行してください。

- `task submit`
  - 現在いる**言語のディレクトリ**のコードをAtCoderに提出します。
  - 問題の言語ディレクトリ (例: `abc123/a/rust`) 内で実行してください。 

## 問題管理

### code repository

atcoderで解いた問題、使えそうなTips、アルゴリズム等をまとめています。

## 問題のレベル
:brown_square: - 茶色レベル
:green_square: - 緑色レベル
:droplet: - 水色レベル
:blue_square: - 青色レベル
:yellow_square: - 黄色レベル
:orange_square: - 橙色レベル
:red_square: - 赤色レベル

## 本番
:sparkles: ??? performance

## 練習
:hammer: practice

### 問題なしの問題
:white_check_mark: かかった時間/推測時間 <<色>>

### 再度解くべき問題
:rotating_light: かかった時間/推測時間 <<色>>

### ２度目
:fire: かかった時間/推測時間 <<色>> 