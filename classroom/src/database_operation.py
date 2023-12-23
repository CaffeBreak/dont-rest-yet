import os
import os.path
from dotenv import load_dotenv
from typing import Union

from prisma import Prisma

from encrypt import decrypt_token


dotenv_path = '../.env'
load_dotenv(verbose=True, dotenv_path=dotenv_path)

DISCORD_BOT_TOKEN = os.getenv("DISCORD_BOT_TOKEN")

CLIENT_SECRETS_FILE = (
    "../credentials.json"
)
SCOPES = ["https://www.googleapis.com/auth/classroom.courses.readonly"]
REDIRECT_URI = "https://izumo-desktop.taila089c.ts.net/callback"

KEY = os.getenv("KEY")


async def delete_all_auth_data() -> bool:
  async with Prisma() as db:
    await db.auth_data.delete_many()
    return True


async def delete_auth_data_by_discord_id(discord_id: int) -> bool:
  async with Prisma() as db:
    discord_id_str = str(discord_id)

    # 既存のレコードを確認
    existing_record = await db.auth_data.find_first(
        where={
            'discord_id': discord_id_str,
        },
    )

    if existing_record:
      # レコードが存在する場合、削除を実行
      await db.auth_data.delete_many(
          where={
              'discord_id': discord_id_str,
          },
      )
      return True
    else:
      # レコードが存在しない場合
      return False


async def save_auth_data(encrypted_token: str, discord_id: int) -> bool:
  async with Prisma() as db:
    discord_id_str = str(discord_id)

    # 既存のレコードを確認
    existing_record = await db.auth_data.find_first(
        where={
            'discord_id': discord_id_str,
        },
    )

    if existing_record:
      return False
    else:
      # 新しいレコードを作成
      await db.auth_data.create(
          data={
              'google_token': encrypted_token,
              'discord_id': discord_id_str
          },
      )
      return True


async def get_refresh_token_by_discord_id(discord_id: int) -> Union[str, bool]:
  async with Prisma() as db:
    # 指定された discord_id を持つ auth_data レコードを取得
    auth_data_record = await db.auth_data.find_first(
        where={
            'discord_id': str(discord_id),
        },
    )
    # auth_data レコードが存在すればそのトークンを返す
    if auth_data_record:
      return decrypt_token(auth_data_record.google_token)
    else:
      return False
