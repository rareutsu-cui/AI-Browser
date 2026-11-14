# GitHub Actions で Linux deb を作る

手元の Rust ビルド用ディスクと CPU の負担を減らすための、手動実行専用ワークフロー。

## 使い方

1. 自分の GitHub リポジトリの Actions を開く。
2. 左側の Linux deb build を選ぶ。
3. Run workflow を開く。ワークフローを置いたブランチを選ぶ。
4. source_ref は通常 fix/linux-webview-layout。特定コミットも指定できる。
5. 実行後、成功した実行の Artifacts から ai-browser-linux-amd64-実行番号をダウンロードする。
6. ZIP を展開し、sha256sum -c SHA256SUMS で照合してから、実機で動作を確認する。

出力は deb、SHA256SUMS、原ライセンス、BUILD-INFO.txt。BUILD-INFO.txt に実際にビルドしたコミットと環境を記録する。

自動 Release 作成、添付済み配布物の置き換え、Cloudflare でのアプリビルドは行わない。Artifacts は14日で期限切れになるため、採用する配布物は別途保存する。

## 環境

Ubuntu 24.04 amd64、Node.js 22、Rust stable。GTK／WebKitGTK の開発依存をインストールして、npm ci、型検査、Rust テスト、deb ビルドを実行する。

手元の Ubuntu とはライブラリのバージョンが異なる可能性がある。CI 成功だけでは GUI・ログイン・エディタ送信などの動作確認は完了しない。

## 導入時

GitHub の手動実行ボタンに表示するため、このワークフローはデフォルトブランチ main にも登録する。ビルド対象は source_ref で明示し、main の古いアプリを誤ってビルドしない。

Fork の Actions が無効の場合はリポジトリ所有者が有効化する必要がある。機密値や署名鍵は不要。手元のビルド環境の削除は、CI 生成物の実機確認後に判断する。
