# ベースイメージ
FROM rust:latest as builder

# 作業ディレクトリの設定
WORKDIR /usr/src/app

# ソースコードのコピー
COPY . .

# 依存関係のインストールとビルド
RUN cargo install --path .

# リリースビルド
RUN cargo build --release

# 実行用のスリムなイメージを作成
FROM debian:bookworm-slim

# 必要なライブラリのインストール
RUN apt-get update\
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# 作業ディレクトリの設定
WORKDIR /usr/src/app

# ビルドしたバイナリをコピー
COPY --from=builder /usr/src/app/target/release/randam_icon /usr/local/bin/randam_icon

# ポートの公開
EXPOSE 8080

# コンテナ起動時に実行するコマンド
CMD ["randam_icon"]
