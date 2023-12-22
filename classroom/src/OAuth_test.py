import os
import os.path
import uuid
import threading
from dotenv import load_dotenv
from typing import Union

import discord
from aiohttp import web
from discord.ext import commands
from google_auth_oauthlib.flow import Flow
from prisma import Prisma

from encrypt import encrypt_token, decrypt_token
from database_operation import save_auth_data, get_token_by_discord_id, delete_all_auth_data, delete_auth_data_by_discord_id

dotenv_path = '../.env'
load_dotenv(verbose=True, dotenv_path=dotenv_path)

DISCORD_BOT_TOKEN = os.getenv("DISCORD_BOT_TOKEN")

CLIENT_SECRETS_FILE = (
    "../credentials.json"
)
SCOPES = ["https://www.googleapis.com/auth/classroom.courses.readonly"]
REDIRECT_URI = "https://izumo-desktop.taila089c.ts.net/callback"

KEY = os.getenv("KEY")

intents = discord.Intents.default()  # デフォルトのintentsを取得
intents.message_content = True  # メッセージの内容を読み取るためのintentを有効にする

bot = commands.Bot(command_prefix="!", intents=intents)  # intents引数を提供


@bot.event
async def on_ready_send_message():
  if bot.user is not None:
    print(f"{bot.user.name} has connected to Discord!")


# 一時認証コードからdiscord_user_idを取得するのに使う{[discord_user_id;一時認証コード]}
auth_codes: dict[str, int] = {}

# !auth のコマンドで OAuth の URL をユーザーにDM送付


@bot.command(name="auth")
async def start_auth(ctx: commands.Context):
  # 認証ファイルからURLのオブジェクト作成
  flow = Flow.from_client_secrets_file(

      CLIENT_SECRETS_FILE, scopes=SCOPES, redirect_uri=REDIRECT_URI
  )

  # uuidを一時コードとしてdiscord id と一緒に保存
  temp_code = str(uuid.uuid4())
  auth_codes[temp_code] = str(ctx.author.id)

  # stateパラメータを指定して認証URLを生成
  auth_url, _ = flow.authorization_url(prompt="consent", state=temp_code)

  # コマンドを送ってきたユーザーにURLをDMで送る
  user = ctx.author
  await user.send(f"Click on the link to authenticate: {auth_url}")


@bot.command(name="auth_delete")
async def delete_auth_data_by_discord_id_receve(ctx: commands.Context):
  user = ctx.author
  TorF_delete_auth_data_by_discord_id: bool = await delete_auth_data_by_discord_id(int(user.id))
  if TorF_delete_auth_data_by_discord_id is True:
    await user.send("削除に成功しました")
  else:
    await user.send("削除対象がありませんでした")
# envから取ってきたkeyで暗号化


@bot.command(name="all_auth_delete")
async def delete_all_auth_data_command(ctx: commands.Context):
  user = ctx.author

  # 管理者権限の確認や、特定のユーザーのみが実行できるようにするなどのセキュリティチェックを追加することをお勧めします
  # 例: if user.id == <管理者のユーザーID>:

  success = await delete_all_auth_data()
  if success:
    await user.send("すべての認証データが削除されました")
  else:
    await user.send("認証データの削除に失敗しました")


async def handle_callback(request):
  code = request.query.get("code")
  temp_code = request.query.get("state")  # 一時コードを取得
  user_id = auth_codes.pop(temp_code, None)  # 一時コードのdiscord id を取得し削除

  if user_id is None:
    # 一時コードが無効な場合、エラーメッセージを表示
    return web.Response(text="認証が失敗したので、もう一度試すか、管理者に問い合わせてください。")

  flow = Flow.from_client_secrets_file(CLIENT_SECRETS_FILE,
                                       scopes=SCOPES,
                                       redirect_uri=REDIRECT_URI)
  flow.fetch_token(code=code)

  credentials = flow.credentials
  token = credentials.token
  refresh_token = credentials.refresh_token

  print(refresh_token)

  encrypted_token = encrypt_token(refresh_token)

  TorF_save_auth_data = await save_auth_data(encrypted_token, user_id)

  async with Prisma() as db:
    auth_data_record = await db.auth_data.find_first(
        where={
            'discord_id': user_id,
        },
    )
    # auth_data レコードが存在すればそのトークンを返す
    if auth_data_record:
      print(decrypt_token(auth_data_record.google_token))
    else:
      print("Token not found for the specified discord_id.")

  print(refresh_token)
  print(user_id)
  print(token)

  if TorF_save_auth_data == True:
    return web.Response(text="認証が成功しました。このウィンドウを閉じてください。")
  else:
    return web.Response(text="discord_idに紐づけられたトークンが既に登録されているか、登録に失敗しました。")


def run_discord_bot():
  bot.run(DISCORD_BOT_TOKEN)


def run_local_server():
  app = web.Application()
  app.router.add_get("/callback", handle_callback)
  web.run_app(app, port=8080)


def main():
  # Discord ボットを別のスレッドで実行
  bot_thread = threading.Thread(target=run_discord_bot)
  bot_thread.start()

  # Web サーバーを現在のスレッドで実行
  run_local_server()


if __name__ == "__main__":
  main()
