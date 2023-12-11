import os
import os.path
import sys
import uuid
import threading
import discord
from aiohttp import web
from discord.ext import commands
from dotenv import load_dotenv
from google_auth_oauthlib.flow import Flow

# pipでパスが通らなかったときの名残
sys.path.append("./myenv/lib/site-packages")

dotenv_path = '../.env'
load_dotenv(verbose=True, dotenv_path=dotenv_path)

DISCORD_BOT_TOKEN = os.getenv("DISCORD_BOT_TOKEN")

CLIENT_SECRETS_FILE = (
    "../credentials.json"
)
SCOPES = ["https://www.googleapis.com/auth/classroom.courses.readonly"]
REDIRECT_URI = "https://izumo-desktop.taila089c.ts.net/callback"

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
  print(user_id)
  print(token)

  return web.Response(text="認証が成功しました。このウィンドウを閉じてください。")


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
