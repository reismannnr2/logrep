# Logrep: ブロック指向grep

## 概要

正規表現で記述されたデリミタで区切られた"ブロック"単位で検索・出力する単一ファイルgrepです。

仕事で数十万行のログから不具合の原因を探すことが多いので作りました。

## 用途

```
[YYYY/MM/DD HH:MM:SS.ZZZ GMT] ERROR SUMMARY LINE
BLA BLA BLA
FOO BAR BAZ
HOGE FUGA PIYO
[YYYY/MM/DD HH:MM:SS.ZZZ GMT] DEBUG SUMMARY LINE
DER DES DEM DEN
DIE DER DER DIE
DAS DES DEM DAS
```

以上のようなログを `FOO` で検索したら、

```
FOO BAR BAZ
```

**ではなく**

```
[YYYY/MM/DD HH:MM:SS.ZZZ GMT] ERROR SUMMARY LINE
BLA BLA BLA
FOO BAR BAZ
HOGE FUGA PIYO
```

というひとまとまりのログ単位を出力させるためのものです。

## 基本

    logrep -d <delimiter> <pattern> [filename]

`<delimiter>` が区切り行、 `<pattern>` が検索したい文字列、 `[filename]` が検索対象ファイル。

区切り行は常に正規表現として解釈されます。標準入力にも対応しています。

    cat file | logrep -d <delimiter> <pattern>

`-d <delimiter>` は省略可能です。 `LOGREP_DELIMITER` が環境変数にあればその内容を、なければ空行もしくは空白行をデフォルト値として使います。

## 使い方:

    logrep [FLAGS] [OPTIONS] <pattern> [filename]

### フラグ:

    -e, --exclude        1行でもマッチしたブロックを除外して出力します。
    -h, --help           ヘルプを表示します。
    -i, --ignore-case    大文字小文字の違いを無視して検索します。
    -r, --regex          正規表現で検索を行います。
    -V, --version        プログラムのバージョンを表示します。

### オプション:

    -d, --delimiter <delimiter>    正規表現で明示的に区切り行を指定します。
                                    [環境変数: LOGREP_DELIMITER=]  [デフォルト: ^\s*$]

### 引数:

    <pattern>     検索対象の文字列です。
    <filename>    検索対象のファイルです。省略した場合は標準入力から読み込みます。

