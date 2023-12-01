from operator import ne
import signal
import sys
import discord
from remindcmd import Remindcmd
from discord import Intents, Client, abc
from pb.dry import reminder
from grpclib.client import Channel
from discord.app_commands import CommandTree
import os
from const import JST
import random
import asyncio
from dotenv import load_dotenv
import json
from google.protobuf.empty_pb2 import Empty
from typing import Any, Callable,Coroutine
from datetime import datetime, timedelta
from view import bottonView
from ResponseSet import response_sets


dotenv_path = '../.env'
load_dotenv(verbose=True, dotenv_path=dotenv_path)


token = os.getenv("TOKEN")
GUILD_ID = os.getenv("GUILD_ID")

with open("config.json", "r") as file:
  data = json.load(file)
  channel_id = data.get("channel_id", None)

class MyClient(Client):
  def __init__(self, intents: Intents) -> None:
    super().__init__(intents=intents)
    self.tree = CommandTree(self)
    self.tree.add_command(Remindcmd("remind"))
  async def on_ready(self):
    loop = asyncio.get_event_loop()
    loop.create_task(Notification(channel_id))
    print("起動完了")
  async def setup_hook(self) -> None:
    print("同期開始")
    if not GUILD_ID:
      print("ギルドIDがありません")
      return
    await self.tree.sync(guild=discord.Object(id=GUILD_ID))
    print("同期完了")
  async def on_close(self) -> None:
    print("終了します")

    
async def Notification(channel_id):
  channel = client.get_channel(channel_id)
  if not isinstance(channel, abc.Messageable):
    print("通知の送信先がメッセージを送信可能なチャンネルではありません")
    
    sys.exit(1)
  
  channels = Channel(host= "reminder", port=58946)
  service1 = reminder.NotificationServiceStub(channels)
  service2 = reminder.TaskServiceStub(channels)
  
  print("通知待機開始")

  async for response in service1.push_notification(betterproto_lib_google_protobuf_empty=Empty()):
    print("タスク受け取り中")
    request = reminder.UpdateTaskRequest(
    id= response.id,
    title= response.title,
    remind_at= response.remind_at + timedelta(days=1)
    )
    selected = random.choice(response_sets)
    update_task: Callable[[],Coroutine[Any, Any, reminder.Task]] = lambda : service2.update_task(request)
    view = bottonView(update_task,response.title,selected.yes,selected.no)
    await channel.send(content= f"<@{response.who}>{selected.message.format(title = response.title)}",view=view,)

  channels.close()

def signal_handler(signum, frame):
    print("終了します")
    channel = Channel(host= "reminder", port=58946)
    channel.close()
    sys.exit()


intents = Intents.default()
client = MyClient(intents=intents)

signal.signal(signal.SIGINT, signal_handler)

if token:
    client.run(token=token)
else:
    print("Token is not available.")
