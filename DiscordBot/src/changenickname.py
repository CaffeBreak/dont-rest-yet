import discord
from discord import Intents, Client
import os
from const import JST
from dotenv import load_dotenv
import json
from datetime import datetime
from discord.ext import tasks

dotenv_path = '../.env'
load_dotenv(verbose=True, dotenv_path=dotenv_path)


token = os.getenv("TOKEN")
GUILD_ID = os.getenv("GUILD_ID")


class MyClient(Client):
  def __init__(self, intents: Intents) -> None:
    super().__init__(intents=intents)
  async def on_ready(self):
    change_nickname.start()
    print("起動完了")   
  async def on_close(self) -> None:
    print("終了します")

intents = Intents.default()
client = MyClient(intents=intents)

@tasks.loop(minutes=30)
async def change_nickname():
    print("ユーザー名変更開始")
    if not GUILD_ID:
      print("ギルドIDがありません")
      return
    with open("nikiname.json", "r") as file:
      data = json.load(file)
      (old_nickname) = data.get("old_nickname", None)
    current_time = datetime.now(JST).hour
    if 6 <= current_time < 10:
        new_nickname = "朝食勇気"
    elif 10 <= current_time < 15:
        new_nickname = "昼食勇気"
    elif 15 <= current_time < 16:
        new_nickname = "間食勇気"
    elif 16 <= current_time < 20:
        new_nickname = "夕食勇気"
    else:
        new_nickname = "夜食勇気"
        
    print(f"現在の時間は{current_time}")
    
    print(f"現在{old_nickname}です")
        
    print(f"{new_nickname}に変更します")
        
    if new_nickname == old_nickname:
      print(f"既に{old_nickname}です")
      return
    
    user_id = 442273058670247939 # 対象のユーザーのID # サーバーのIDを設定
    
    guild = client.get_guild(int(GUILD_ID))
    if not guild:
      print("ギルドがありません")
      return

    member = await guild.fetch_member(user_id)
    if not member:
      print("メンバーありません")
      return
    
    try:
      await member.edit(nick=new_nickname)
      print(f"{new_nickname}にサーバー{guild.name}で変更しました")
      with open("nikiname.json", "w") as file:
        json.dump({"old_nickname": new_nickname}, file)
    except discord.Forbidden:
        print(f"Bot does not have permission to change nickname for {member.display_name} in {guild.name}")
    except Exception as e:
        print(f"An error occurred while changing nickname for {member.display_name} in {guild.name}: {e}")


if token:
    client.run(token=token)
else:
    print("Token is not available.")
