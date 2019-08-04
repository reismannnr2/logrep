# Logrep: Block-oriented mini-grep tool

## BASICS:

Grep-like mini-tool which split text into blocks by delimiter.

It prints out blocks any of its lines matches the given pattern.

## WHAT IT DOES:

Here is sample text:

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

If you input the command `grep FOO` then, the following will be displayed:

```
FOO BAR BAZ
```

Logrep will prints out the following instead:

```
[YYYY/MM/DD HH:MM:SS.ZZZ GMT] ERROR SUMMARY LINE
BLA BLA BLA
FOO BAR BAZ
HOGE FUGA PIYO
```

## USAGE:

    logrep.exe [FLAGS] [OPTIONS] <pattern> [filename]

## FLAGS:

    -e, --exclude        exclude block if any of its lines matches pattern
    -h, --help           Prints help information
    -i, --ignore-case    search case-insensitively
    -r, --regex          search by regex
    -V, --version        Prints version information

## OPTIONS:

    -d, --delimiter <delimiter>    explicit delimiter pattern
                                    [env: LOGREP_DELIMITER=]  [default: ^\s*$]

## ARGS:

    <pattern>     pattern to search
    <filename>    target file. use stdin if omitted
