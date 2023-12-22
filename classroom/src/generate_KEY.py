import os
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import hashes
from cryptography.fernet import Fernet, InvalidToken
from base64 import urlsafe_b64encode
import base64
from dotenv import load_dotenv

dotenv_path = '../.env'
load_dotenv(verbose=True, dotenv_path=dotenv_path)

PASSPHRASE = os.getenv("PASSPHRASE")
SALT = base64.b64decode(os.getenv("SALT"))

# 共通鍵を生成する関数


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

# メッセージを暗号化する関数


def encrypt_message(message: str, key: bytes) -> bytes:
  f = Fernet(key)
  encrypted_message = f.encrypt(message.encode())
  return encrypted_message

# 暗号化されたメッセージを復号化する関数


def decrypt_message(encrypted_message: bytes, key: bytes) -> str:
  f = Fernet(key)
  try:
    decrypted_message = f.decrypt(encrypted_message).decode()
    return decrypted_message
  except InvalidToken:
    raise ValueError("Invalid key - decryption failed.")


# 例
print(SALT)
key = generate_key(PASSPHRASE, SALT)
key = os.getenv("KEY")
# トークンを暗号化
token = "secret_token"
encrypted_token = encrypt_message(token, key)

# トークンを復号化
try:
  decrypted_token = decrypt_message(encrypted_token, key)
except ValueError as e:
  decrypted_token = str(e)

print(encrypted_token)
print(decrypted_token)
