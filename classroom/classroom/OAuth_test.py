from __future__ import print_function

import os
import os.path
import sys
import uuid
from os.path import dirname, join

import discord
from aiohttp import web
from discord.ext import commands
from dotenv import load_dotenv
from google_auth_oauthlib.flow import Flow

# pipでパスが通らなかったときの名残
sys.path.append(
    "./myenv/lib/site-packages"
)


dotenv_path = join(dirname(__name__), ".env")
load_dotenv(verbose=True, dotenv_path=dotenv_path)

CLIENT_SECRETS_FILE = (
    "./credentials.json"
)
SCOPES = ["https://www.googleapis.com/auth/classroom.courses.readonly"]
REDIRECT_URI = "http://localhost:8080/callback"

intents = discord.Intents.default()  # デフォルトのintentsを取得
intents.message_content = True  # メッセージの内容を読み取るためのintentを有効にする

bot = commands.Bot(command_prefix="!", intents=intents)  # intents引数を提供


@bot.event
async def on_ready_send_message():
  print(f"{bot.user.name} has connected to Discord!")


auth_codes: dict[int] = {}  # 一時認証コードからdiscord_user_idを取得するのに使う{[discord_user_id;一時認証コード]}

# !auth のコマンドで OAuth の URL をユーザーにDM送付


@bot.command(name="auth")
async def start_auth(ctx):
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


async def handle_callback(request):
  code = request.query.get("code")
  temp_code = request.query.get("state")  # 一時コードを取得
  user_id = auth_codes.pop(temp_code, None)  # 一時コードのdiscord id を取得し削除

  if user_id is None:
    # 一時コードが無効な場合、エラーメッセージを表示
    return web.Response(text="認証が失敗したので、もう一度試すか、管理者に問い合わせてください。")

  flow = Flow.from_client_secrets_file(
      CLIENT_SECRETS_FILE, scopes=SCOPES, redirect_uri=REDIRECT_URI
  )
  flow.fetch_token(code=code)

  credentials = flow.credentials
  token = credentials.token
  refresh_token = credentials.refresh_token

  print(refresh_token)
  print(user_id)
  print(token)

  return web.Response(text="認証が成功しました。このウィンドウを閉じてください。")


def run_local_server():
  app = web.Application()
  app.router.add_get("/callback", handle_callback)
  web.run_app(app, port=8080)


if __name__ == "__main__":

  run_local_server()

  # Run the Discord bot
  bot.run(os.getenv("TOKEN"))
