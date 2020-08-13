# ラズペリーパイ Remote Car project

##これは何？
ラズベリーパイでロボット・カーを作成するプロジェクトです。

・車管理用 Web UI
・サーボサーバー
　　・コマンド受付サーバー
　　・中間レイヤー
　　・ハードウェア制御

で構成されています

コマンド受付サーバーはUDPで受付ます。


## install 

###必要な外部ライブラリ
sudo pip install rpi.gpio

###サーバー起動
python ./bin/go.py

###webアクセス
ブラウザでweb/index.phpを開く




###Feature
1.wifi IP経由でコマンドを受け付けているのをBluetooth経由通信もできるようにする



