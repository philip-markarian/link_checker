# Link Checker V1

## link_checkerの使い方

リポジトリをダウンロードし、フォルダにCSVファイルを置く。コマンドラインで下記のコマンドを実行する。

### コマンド

```
./bin/link_checker <input-file> <output-file> <number-of-columns>
```

### コマンド説明
- input-file：入力となるCSVファイルのパスを指定する。これは、処理する対象のCSVファイル
- output-file：出力先のCSVファイルのパスを指定する。処理結果を出力する際に、このファイルに書き込まれる
- number-of-columns：CSVファイルの列数を指定する。この数値が指定されると、処理中にこのCSVファイルの列数を確認し、正しい列数であることを確認する

### 例：
  
 ```
  ./bin/link_checker links.csv links_checked.csv 2
 ```
  
## CSVの形

```
リンク１,リンク２,リンク３
https://github.com/,https://github.com,https://github.com/
```
  
