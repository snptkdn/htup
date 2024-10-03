import requests
import os
import webbrowser
import subprocess
from os.path import expanduser
from urllib.parse import urlparse
from datetime import datetime

def download_wav(response, directory="~/.config/htup/media"):
    # ディレクトリが存在しない場合は作成
    if not os.path.exists(expanduser(directory)):
        os.makedirs(expanduser(directory))

    # レスポンスのContent-Typeをチェック
    if response.headers.get('Content-Type') != 'audio/wav':
        print(f"警告: Content-Typeが'audio/wav'ではありません。実際の値: {response.headers.get('Content-Type')}")

    # ファイル名が空の場合、タイムスタンプを使用
    filename = f"audio_{datetime.now().strftime('%Y%m%d_%H%M%S')}.wav"

    # ファイルパスを作成
    filepath = os.path.join(expanduser(directory), filename)

    # コンテンツをファイルに書き込み
    with open(filepath, 'wb') as f:
        f.write(response.content)

    print(f"ファイルを保存しました: {filepath}")

    # ダウンロードディレクトリを開く
    # webbrowser.open('file://' + os.path.realpath(directory))
    # Windowsのエクスプローラーでディレクトリを開く
    windows_path = filepath.replace('/mnt/c', 'C:').replace('/', '\\')
    subprocess.run(['explorer.exe', '/select,', windows_path])

