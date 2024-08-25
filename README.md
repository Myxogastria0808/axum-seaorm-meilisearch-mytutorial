# Setup

1. package を追加

```sh
cargo add meilisearch-sdk
cargo add tokio -F full
cargo add serde
```

2. docker-compose.yaml を作成

```yaml
version: "3.8"

services:
  meilisearch:
    container_name: meilisearch
    image: "getmeili/meilisearch:prototype-japanese-184"
    volumes:
      - ./meili_data:/meili_data
    environment:
      #meili cloudを使わない場合、マスターキーは自分で設定する
      - MEILI_MASTER_KEY=master_key
      - MEILI_ENV=development
    ports:
      #データ部分をマウント
      - "7700:7700"
    tty: true
```

3. Default Search API Key の取得

参考サイト 2. より引用

```sh
sudo apt install jq
curl   -X GET 'http://localhost:7700/keys'   -H 'Authorization: Bearer KSjeon19dn3Ls93nFNl349FNS93nkljasIk39fnsa' | jq
```

4. client の生成

```rust
    let client: Client = Client::new(
        "http://127.0.0.1:7700",
        Some("<Default Search API Key>"),
    )
    .unwrap();
```

## 参考サイト

1. https://github.com/meilisearch/meilisearch-rust

2. https://virment.com/setup-masterkey-and-apikey-for-meilisearch/
