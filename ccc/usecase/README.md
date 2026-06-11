# usecase

## 概要

アプリケーションの操作単位 (ユースケース) を定義する crate です。

「式を計算する」「パイプ入力を処理する」「REPL を回す」を提供します。

## 背景

CLI には複数の入力モードがあります。

引数で式を受け取るモード、stdin から行を読むモード、対話モードです。

モードごとの制御フローを presentation に書くと main が肥大化します。
かといって infrastructure に書くと、実装詳細と業務フローが混ざります。

そこで、フローだけをこの crate に切り出しています。

## 目的

- 「parse → type check → evaluate」の手順を1箇所で定義する
- 入力モードごとの制御フローを部品化する
- 具体的な実装ではなく domain の trait に依存する (依存性逆転)

## 手法

ユースケースはジェネリクスで trait 実装を受け取ります。

```rust
pub struct CalculateMathExpressionUsecase<P, T, E> {
    parser: P,
    type_checker: T,
    evaluator: E,
}
```

どのパーサー・評価器を使うかは呼び出し側 (presentation) が注入します。
この crate は infrastructure を知りません。

### 主要モジュール

| モジュール | 内容 |
|---|---|
| `calculate_math_expression` | parse → type_check → evaluate の直列実行 |
| `evaluate_piped_input` | stdin の行ごとの評価。引数サフィックスの付加にも対応 |
| `interactive_repl` | 対話ループ |
| `format_error` | エラー位置をキャレット (`^`) で指すメッセージ整形 |

## 処理の事例

式の計算は3ステップの直列です。
途中で失敗したら、そこで `Err` を返します (鉄道志向)。

```rust
pub fn execute(&self, input: &str) -> Result<Value, CccError> {
    let ast = self.parser.parse(input)?;
    self.type_checker.check(&ast)?;
    self.evaluator.evaluate(&ast)
}
```

エラー整形は発生位置をキャレットで示します。

```text
  *3
  ^
  error: parse error: expected number, function call, list, or '('
```
