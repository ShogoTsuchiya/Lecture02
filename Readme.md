# 第２回 AWS 勉強会

## １．注意点

<ol>

こちらは、第２回 AWS 勉強会用に作成した Actix プロジェクトになります。

各処理におけるエラーハンドリングや、パスワードの暗号化処理等はおこっておりません。

実運用される際は、必要なエラーハンドルの実装や Argon2 などのハッシュ化アルゴリズムを導入するなどしてください。

Actix で使用している Rust という言語は、実行する前にコンパイル（機械語への翻訳）が必要です。

勉強会の中で実行させたコンパイル後のバイナリファイルは `data/samplepage` であり、その素になっているプログラムは、`data/project/` ディレクトリ内に格納されています。

また `actix_cognito` を使用する際は、 `actix_cognito/data/project/src/main.rs` 内の 41 行目と 42 行目をご自身の環境に合わせた適切な内容に変更してください。

<span style="color:red">

※ 将来的にリポジトリを削除する可能性がある点だけご了承ください。

※ あくまでも、 EC2 インスタンスと、データベースおよび AWS リソースとの連携の仕方等を体験・説明する目的で作成しているアプリケーションです。

</span>

</ol>

## ２．全体の流れ

<ol>

２．１．複数のWebサーバー起動

<ol>

２．１．１．第１フロー（パブリックサブネット内のデータベースを構築）

<ol>

まず、データベースのパブリックサブネットグループを作成してから、クラスターも作成する。

<ol>

| 項目             | 値       |
| ---------------- | -------- |
| ルートユーザー名 | shogo    |
| パスワード       | password |

</ol>

クラスターの作成中、以下のコマンドを使用して第１回勉強会で使用した Web サイトを起動する。

<ol>

```sh
# EC2 インスタンスに SSH ログイン（Windows コマンドプロンプト／事前にユーザーディレクトリに鍵ファイルを移動）
ssh -i 鍵ファイル名 ec2-user@EC2インスタンスのパブリックIPアドレス

# CD を変更
cd Lecture01/

# ビルドしたイメージからコンテナを起動（この後なら、ウェブブラウザから閲覧が可能）
sudo docker run --name weatherapp -d -p 8080:8080 --rm weather
```

</ol>

その後、以下のコマンドで第２回勉強会用のプロジェクトパッケージを[ここ](https://github.com/ShogoTsuchiya/Lecture02)から取得し、`actix` コンテナを起動させる。

<ol>

```sh
# 専用 Git リポジトリをクローン
git clone https://github.com/ShogoTsuchiya/Lecture02.git

# CD を変更
cd Lecture02/actix

# 専用イメージのビルド
sudo docker build --no-cache -t actix .

# 立てるコンテナ用のネットワーク作成
sudo docker network create --subnet=172.111.0.0/24 lecture_network

# ビルドしたイメージからコンテナを起動（事前にクラスターからホスト名を確認）
sudo docker run --name actixapp -it -d -e DATABASE_URL=mysql://shogo:password@<ホスト名>:3306/mydb -p 80:80 --net lecture_network --ip 172.111.0.3 --rm actix
```

</ol>

そして、以下のコマンドでクラスターのIPアドレスを確認し、[このサイト](https://nishinatoshiharu.com/fundamental-ipaddress/)からパブリック IP アドレスであることも確認する。

<ol>

```sh
traceroute <ホスト名>
```

</ol>

ここまで来たら、A5M2 等の DB クライアントアプリ等から以下の SQL を実行し、必要なデータベースとテーブルを作成する。

<ol>

```sql
CREATE DATABASE mydb CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_bin;
USE mydb;
CREATE TABLE users (
    username VARCHAR(100) NOT NULL,
    password VARCHAR(100) NOT NULL
);
```

</ol>

データベースの準備が整ったら、Actix サイトから情報を登録し、正しく情報が格納されたことを確認する。

</ol>


２．１．２．第２フロー（プライベートサブネット内にデータベースを構築）

<ol>

まず、後で使用する EC2 インスタンスを先に起動させてからプライベートサブネットグループを作成。

事前にお３方様の２つ目のプライベートサブネットを土屋で作成済みなので、それらのサブネットを使用してグループを作成。

その後、以下の情報を入れてクラスターを作成。

<ol>

| 項目             | 値       |
| ---------------- | -------- |
| ルートユーザー名 | shogo    |
| パスワード       | password |

</ol>

クラスター作成中、EC2インスタンスで以下の操作をして準備を進める。

<ol>

```sh
# EC2 インスタンスに SSH ログイン（Windows コマンドプロンプト／事前にユーザーディレクトリに鍵ファイルを移動）
ssh -i 鍵ファイル名 ec2-user@EC2インスタンスのパブリックIPアドレス

# 専用 Git リポジトリをクローン
git clone https://github.com/ShogoTsuchiya/Lecture02.git

# CD を変更
cd Lecture02/actix

# 専用イメージのビルド
sudo docker build --no-cache -t actix .

# 立てるコンテナ用のネットワーク作成
sudo docker network create --subnet=172.111.0.0/24 lecture_network

# ビルドしたイメージからコンテナを起動（事前にクラスターからホスト名を確認）
sudo docker run --name actixapp -it -d -e DATABASE_URL=mysql://shogo:password@<ホスト名>:3306/mydb -p 80:80 --net lecture_network --ip 172.111.0.3 --rm actix
```

</ol>

そして、以下のコマンドでクラスターのIPアドレスを確認し、[このサイト](https://nishinatoshiharu.com/fundamental-ipaddress/)からプライベート IP アドレスであることも確認する。

<ol>

```sh
traceroute <ホスト名>
```

</ol>

その後、連携させるデータベースを準備するために MySQL クライアント用のコンテナを立ち上げログインする。

<ol>

```sh
# MySQL クライアント用のコンテナを起動
sudo docker run -d --name mysql --rm --tty -e MYSQL_ROOT_PASSWORD=SamplePW mysql:8.1

# コンテナでクライアントを実行
sudo docker exec -it mysql mysql -h <ホスト名> -u shogo -ppassword
```

</ol>

ログイン後、以下の SQL を実行し、データベースとテーブルの作成をおこなう。

<ol>

```sql
CREATE DATABASE mydb CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_bin;
USE mydb;
CREATE TABLE users (
    username VARCHAR(100) NOT NULL,
    password VARCHAR(100) NOT NULL
);
```

</ol>

その後、Web ブラウザを展開して情報を登録＋登録された情報をクライアントから確認する。

</ol>

２．１．３．第３フロー（Cognitoサービスの利用）

<ol>

まず、以下の情報（一部抜粋）で Cognito ユーザープールを作成する。

<ol>

| 項目名                                       | 値                     |
| -------------------------------------------- | ---------------------- |
| Cognito ユーザープールのサインインオプション | Eメール                |
| パスワードポリシー                           | デフォルト             |
| MFA                                          | なし                   |
| アカウント復旧                               | なし                   |
| 自己登録を有効化                             | true                   |
| 必須の属性                                   | email, name            |
| Eメールプロバイダー                          | CognitoでEメールを送信 |
| アプリケーションタイプ                       | 秘密クライアント |
| アプリケーションクライアント名               | lecture                |

</ol>

その後、以下のコマンドを実行。

<ol>

```sh
# コンテナ群の停止
sudo docker stop weatherapp actixapp mysql

# イメージキャッシュやネットワーク等を削除
sudo docker system prune --volumes --all -f

# actix_cognito に CD を変更
cd ~/Lecture02/actix_cognito

# 専用イメージのビルド
sudo docker build --no-cache -t actix .

# 立てるコンテナ用のネットワーク作成
sudo docker network create --subnet=172.111.0.0/24 lecture_network

# ビルドしたイメージからコンテナを起動（アプリクライアントIDと、クライアントシークレットの情報を参照する。）
sudo docker run --name actixapp -it -d -e CLIENT_ID=アプリクライアントID -e APP_CLIENT_SECRET=クライアントシークレット -p 80:80 --net lecture_network --ip 172.111.0.3 actix
```

</ol>

</ol>

</ol>

</ol>