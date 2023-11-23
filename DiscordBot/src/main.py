from ast import Module
from distutils.cmd import Command
import os
from re import I
import grpc
import discord
from discord import Interaction, app_commands
from dotenv import load_dotenv

from pb import discorbot_pb2_grpc
from pb import discorbot_pb2
from datetime import datetime, timezone
from google.protobuf.timestamp_pb2 import Timestamp

dotenv_path = '../.env'
load_dotenv(verbose=True, dotenv_path=dotenv_path)

intents = discord.Intents.default()

# Discordクライアントを作成します。
client = discord.Client(intents=intents)

# コマンドツリーを作成します。
tree = app_commands.CommandTree(client)

def convert_to_timestamp(year, month, day, hour=0, minute=0, second=0):
    # datetimeオブジェクトを作成
    dt = datetime(year, month, day, hour, minute, second, tzinfo=timezone.utc)

    # datetimeオブジェクトをtimestampに変換
    timestamp = int(dt.timestamp())

    # Timestampメッセージを作成
    timestamp_message = Timestamp(seconds=timestamp)

    return timestamp_message
# 起動イベントを定義します。
@client.event
async def on_ready():
  print("起動完了")
  await tree.sync()# スラッシュコマンドを同期
@client.event 
async def setup_hook() -> None:
  await tree.sync()

    
@tree.command(name="remind_add", description="リマインドを行います")
async def remind(interaction: Interaction, main: str, days: str, time: str):
    """
    main : str
        リマインドしたい内容 
        
    days : str
        リマインドしたい日付(10/31)
    
    time : str
        リマインドする時間(18:00)
    """
    month, day = map(int, days.split('/'))
    hour, minute = map(int, time.split(':'))
    year = datetime.now().year
    Uid = interaction.user.id
    print(Uid)
    result = convert_to_timestamp(year, month, day, hour, minute, 0)
    
    print(type(result))
    print(type(Uid))
    
    request = discorbot_pb2.CreateTaskRequest(
      title=main,
      remindAt=result,
      who=str(Uid)
    )
    with grpc.insecure_channel('reminder:58946') as channel:
        stub = discorbot_pb2_grpc.TaskServiceStub(channel)
        response = stub.CreateTask(request)
    print(response)

      
    await interaction.response.send_message(f"Hi, {main} {day} {time} ")
    
@tree.command(name="remind_list", description="リマインドのリストを表示します")
async def remind_list(interaction: Interaction):
    Uid = interaction.user.id
    request = discorbot_pb2.DeleteTaskRequest(
      id= str(Uid)
    )
    with grpc.insecure_channel('reminder:58946') as channel:
        stub = discorbot_pb2_grpc.TaskServiceStub(channel)
        response = stub.CreateTask(request)
  
    await interaction.response.send_message(f"Hi,")
  

@tree.command(name="remind_delete", description="リマインドの削除を行います")
async def remind_delete(interaction: Interaction, ID: str):
  await interaction.response.send_message(f"Hi, {ID} ")

client.run(token= os.getenv("TOKEN"))
