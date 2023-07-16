# ベースとなるDockerイメージ指定
FROM node:14
# 作業ディレクトリを設定
WORKDIR /app
# ホストのファイルをコピー
COPY package*.json ./
# パッケージのインストール
RUN npm install
COPY . .
# ポート3000番を開放
EXPOSE 3000
# コマンドの実行
CMD [ "npm", "start" ]
