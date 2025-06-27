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

Rustの場合、各問題ディレクトリはそれぞれが独立したCargoプロジェクトとなっており、サンプルケースも自動で `tests` ディレクトリにダウンロードされます。

### 2. 問題を解く

解答したい問題のディレクトリに移動します。

```bash
# cd <コンテストID>/<問題ID>
cd abc123/a
```

`src/main.rs` をエディタで開き、コーディングを開始します。

### 3. 解答をテストする

コードが書けたら、その問題ディレクトリ内で以下のコマンドを実行して、サンプルケースをテストします。

```bash
task test
```

`online-judge-tools` が `tests` ディレクトリ内のサンプルケースをすべて使って、コードの正しさを検証します。

### 4. 解答を提出する

すべてのサンプルケースで正解したら、同じく問題ディレクトリ内で以下のコマンドを実行して、AtCoderに解答を提出します。

```bash
task submit
```

`atcoder-cli` が自動的に `src/main.rs` を提出します。

## Taskfile の使い方

このプロジェクトでは、繰り返し行う操作を `Taskfile.yml` にまとめています。

- `task new -- <contest_id>`
  - 新しいコンテストのディレクトリと、各問題のプロジェクトを生成します。
  - プロジェクトのルートディレクトリで実行してください。
  - (例: `task new -- agc049`)

- `task test`
  - 現在いる問題ディレクトリのコードを、サンプルケースでテストします。
  - 問題ディレクトリ (例: `abc123/a`) 内で実行してください。

- `task submit`
  - 現在いる問題ディレクトリのコードをAtCoderに提出します。
  - 問題ディレクトリ (例: `abc123/a`) 内で実行してください。 