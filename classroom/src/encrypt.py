import os
import os.path

from dotenv import load_dotenv

from cryptography.fernet import Fernet, InvalidToken

dotenv_path = '../.env'
load_dotenv(verbose=True, dotenv_path=dotenv_path)

KEY = os.getenv("KEY")


def decrypt_token(encrypted_token: str) -> str:
  f = Fernet(KEY)
  try:
    decrypted_token_bytes = f.decrypt(encrypted_token.encode('utf-8'))  # UTF-8 エンコーディングを指定
    decrypted_token_str = decrypted_token_bytes.decode('utf-8')  # デコード時も同様に指定
    return decrypted_token_str
  except InvalidToken:
    raise ValueError("Invalid key - decryption failed.")


def encrypt_token(token: str) -> str:
  f = Fernet(KEY)
  encrypted_token_bytes = f.encrypt(token.encode('utf-8'))  # UTF-8 エンコーディングを指定
  encrypted_token = encrypted_token_bytes.decode('utf-8')  # デコード時も同様に指定
  return encrypted_token
