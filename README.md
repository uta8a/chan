# chan
競技プログラミング用CLIツール
**ディレクトリ名など、ローカルなものに依存するので僕以外の環境での動作は想定していません。**

# 使い方
**compile**
```
chan main.cpp
chan main.rs
```
**generate library template**
```
chan g rust hoge
chan g r hoge
```
- generate `uta8alib/src/rust/hoge/main.rs`
```
chan g cpp fuga
chan g c fuga
```
- generate `uta8alib/src/rust/fuga/main.cpp`
