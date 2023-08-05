# RustのNightly版をベースにする
FROM rustlang/rust:nightly AS builder

# 作業ディレクトリを設定
WORKDIR /usr/src/app

# 依存関係をコピーしてビルド
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# ソースコードをコピー
COPY src ./src

# アプリケーションをビルド
RUN cargo build --release

# 新しいステージを作成し、ビルドしたバイナリをコピー
FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/vrc-comppro-connect-backend /usr/local/bin

# ポートを公開
EXPOSE 8080

# 実行コマンドを設定
CMD ["vrc-comppro-connect-backend"]
