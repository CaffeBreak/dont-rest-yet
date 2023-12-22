import os
from dotenv import load_dotenv
import asyncio
from prisma import Prisma
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import hashes
from cryptography.fernet import Fernet, InvalidToken
import base64
from base64 import urlsafe_b64encode

dotenv_path = '../.env'
load_dotenv(verbose=True, dotenv_path=dotenv_path)

PASSPHRASE = os.getenv("PASSPHRASE")
SALT = base64.b64decode(os.getenv("SALT"))
KEY = os.getenv("KEY")


def generate_key(passphrase: str, salt: bytes) -> bytes:
  kdf = PBKDF2HMAC(
      algorithm=hashes.SHA256(),
      length=32,
      salt=salt,
      iterations=100000,
      backend=default_backend()
  )
  key = kdf.derive(passphrase.encode())
  return urlsafe_b64encode(key)


def encrypt_token(token: str) -> str:
  f = Fernet(KEY)
  encrypted_token_bytes = f.encrypt(token.encode())
  encrypted_token = encrypted_token_bytes.decode()
  return encrypted_token


def decrypt_token(encrypted_token: str) -> str:
  f = Fernet(KEY)
  try:
    decrypted_token_bytes = f.decrypt(encrypted_token.encode())
    decrypted_token_str = decrypted_token_bytes.decode()
    return decrypted_token_str
  except InvalidToken:
    raise ValueError("Invalid key - decryption failed.")


async def save_auth_data(encrypted_token: str, discord_id: int) -> None:
  async with Prisma() as db:
    discord_id_str = str(discord_id)

    # 既存のレコードを確認
    existing_record = await db.auth_data.find_first(
        where={
            'discord_id': discord_id_str,
        },
    )

    if existing_record:
      # 既存のレコードがある場合、更新するかエラーメッセージを返す
      print(f'Error: Record with discord_id {discord_id_str} already exists.')
    else:
      # 新しいレコードを作成
      await db.auth_data.create(
          data={
              'google_token': encrypted_token,
              'discord_id': discord_id_str
          },
      )


async def delete_auth_data_by_discord_id(discord_id: int) -> str:
  async with Prisma() as db:
    discord_id_str = str(discord_id)
    result = await db.auth_data.delete_many(
        where={
            'discord_id': discord_id_str,
        },
    )

    if result > 0:
      return f"Deleted {result}record with discord_id {discord_id_str}."
    else:
      return f"No records found with discord_id {discord_id_str} to delete."


async def get_token_by_discord_id(discord_id: int) -> str:
  async with Prisma() as db:
    discord_id_str = str(discord_id)
    auth_data_record = await db.auth_data.find_first(
        where={
            'discord_id': discord_id_str,
        },
    )
    if auth_data_record:
      return decrypt_token(auth_data_record.google_token)
    else:
      return "Token not found for the specified discord_id."


async def delete_all_auth_data() -> None:
  async with Prisma() as db:
    await db.auth_data.delete_many()


async def test_main() -> None:
  sample_discord_id = 1234567890
  sample_token = 'sample_token'
  encrypted_token = encrypt_token(sample_token)
  print(f'encrypted_token {encrypt_token}')

  await save_auth_data(encrypted_token, sample_discord_id)
  print('Token saved successfully.')

  retrieved_token = await get_token_by_discord_id(sample_discord_id)
  print(f'Retrieved Token: {retrieved_token}')

  txt = await delete_auth_data_by_discord_id(sample_discord_id)
  print(txt)
  print('Token deleted successfully.')
  await delete_all_auth_data()

if __name__ == '__main__':
  asyncio.run(test_main())
