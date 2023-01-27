## 計測の流れ
1. サーバー上でHTTPサーバーを起動
2. クライアントから10分間APIコール(vegetaを使用)
3. 計測ツールで停止時間を計測(runtime/debugのGCStatsを使用)

下記詳細

### 1. サーバー上でHTTPサーバーを起動
下記のコードを使用しサーバーを起動
https://github.com/yagipy/althea/tree/main/doc/reference/benchmark/golang
ビジネスロジックを擬似的に再現するために、int64のポインタ配列とフィボナッチ数を使用

### 2. クライアントから10分間APIコール(vegetaを使用)
下記Makefileの`golang-attack`を使用してAPIコールを実施
https://github.com/yagipy/althea/blob/main/doc/reference/benchmark/Makefile

### 3. 計測ツールで停止時間を計測(runtime/debugのGCStatsを使用)
下記のように`/gc-stats`にアクセスし、GCStatsを取得する
```shell
curl <IP>/gc-stats
```

## スペック
### サーバー
- OS：Ubuntu 20.04
- プロセッサ: 2.40GHz 32コア Intel(R) Xeon(R) Gold 6148
- メモリ: 256GB
- ストレージ:2TB

### クライアント
- OS：macOS 12.3.1
- プロセッサ: 2.7GHz 4コア Intel Core i7
- メモリ: 16GB
- ストレージ: 512GB 
