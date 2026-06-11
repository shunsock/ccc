# infrastructure

## 概要

domain の trait を実装する crate です。

パーサー・型検査器・評価器の3つの実装を提供します。

## 背景

「式をどう解析するか」「どう評価するか」は実装の詳細です。

パーサーライブラリの変更や評価戦略の変更が、
上位レイヤーへ波及しない構造が必要でした。

そこで、実装をこの crate に隔離し、
domain の trait 経由でのみ公開しています。

## 目的

- `CccParser` / `CccTypeChecker` / `CccEvaluator` の実装を提供する
- 実装詳細 (pest、評価アルゴリズム) を外部へ漏らさない
- 文法規則・型規則・評価規則を、それぞれ独立に変更可能にする

## 手法

### parser

pest (PEG パーサー) を利用します。

文法は `src/parser/grammar.pest` に宣言的に記述します。

モジュールは文法規則ごとに分かれています。
たとえば duration リテラルの仕様変更は `duration_literal.rs` だけを読めば足ります。

### type_checker

評価前に式の型を推論し、不正な組み合わせを拒否します。

型規則は表として読めるよう、規則の種類ごとにモジュールを分けています
(`binary_rule` / `cast_rule` / `function_rule` / `argument_rule`)。

### evaluator

AST を再帰的に評価します。

二項演算は値の型ペアでディスパッチし、
数値 (`binary_numeric`)・期間 (`binary_duration`)・日時 (`binary_datetime`) に分かれます。

組み込み関数は `builtin/` 配下で分類ごとに実装します
(math / list / statistics / extremum / constructor / time)。

### テスト

`test_fixture` の式ビルダーで AST を構築します。

`add(int(1), mul(int(2), int(3)))` は `1 + 2*3` を表します。

テストは公開 API (trait メソッド) のみを対象とします。

## 処理の事例

`"1 + 2 * 3"` は次のパイプラインで処理されます。

```text
input:  "1 + 2 * 3"
          │
          ▼  PestBasedParser::parse
        AST: Add(1, Multiply(2, 3))
          │
          ▼  AstTypeChecker::check
        OK: Integer 同士の演算
          │
          ▼  AstEvaluator::evaluate
        Value::Integer(7)
```

型検査は不正な式を評価前に弾きます。

```text
input:  "1 + [2]"
          │
          ▼  AstTypeChecker::check
        Err: unsupported operation: integer + list[integer]
```
