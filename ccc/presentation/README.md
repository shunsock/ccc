# presentation

## 概要

CLI のエントリポイントとなる crate です。

バイナリ `ccc` を提供します。

引数の解釈、入力モードの判定、依存の組み立て、出力を担当します。

## 背景

ユーザーとの接点 (CLI 引数、stdin/stdout、終了コード) は
計算ロジックとは変更理由が異なります。

ここを最外殻として分離することで、
内側のレイヤーは入出力を知らずに済みます。

## 目的

- CLI 引数を解釈し、入力モードを決定する
- infrastructure の実装を usecase に注入する (composition root)
- 計算結果とエラーを整形して出力し、終了コードを返す

## 手法

### 入力モードの判定

`input_mode` が引数と stdin の状態からモードを決めます。

| モード | 起動例 |
|---|---|
| `Expression` | `ccc "1 + 2"` |
| `Repl` | `ccc repl` |
| `Pipe` | `echo "1 + 2" \| ccc` |
| `PipeWithArgs` | `echo "1 + 2" \| ccc "* 3"` |
| `ShowBuiltin` | `ccc show builtin` |

### 依存の組み立て

`main.rs` が composition root です。

`PestBasedParser` / `AstTypeChecker` / `AstEvaluator` を生成し、
ユースケースに注入します。

実装の選択がここに集約されるため、
差し替えは main の1箇所の変更で済みます。

### 組み込み関数リファレンス

`builtin_reference.rs` に関数一覧を宣言的なデータとして持ちます。

`show_builtin.rs` はそれを整形して表示するだけです。

### テスト

`tests/cli/` に統合テストがあります。

実際のバイナリを起動し、stdout・stderr・終了コードを検証します。

## 処理の事例

```bash
$ ccc "2 + 3 * 4"
14

$ echo "10 - 3" | ccc
7

$ echo "5" | ccc + 1
6

$ ccc "*3"
  *3
  ^
  error: parse error: expected number, function call, list, or '('
# 終了コード 1
```
