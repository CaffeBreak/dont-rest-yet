import signal
import sys
from remindcmd import Remindcmd
from discord import Intents, Client
from pb.dry import reminder
from grpclib.client import Channel
from discord.app_commands import CommandTree
import os
import asyncio
from dotenv import load_dotenv
from google.protobuf.empty_pb2 import Empty



dotenv_path = '../.env'
load_dotenv(verbose=True, dotenv_path=dotenv_path)

token = os.getenv("TOKEN")

async def Notification():
  print("通知待機開始")
  channels = Channel(host= "reminder", port=58946)
  service = reminder.NotificationServiceStub(channels)
  async for response in service.push_notification(betterproto_lib_google_protobuf_empty=Empty()):
    print("タスク受け取り中")
    print(response)
    channel = client.get_channel(1178934475363713075)
    await channel.send(content= f"<@{response.who}>『{response.title}』をやりましたか？まだ休んではだめですよ")
  channels.close()

def signal_handler(signum, frame):
    print("終了します")
    channel = Channel(host= "reminder", port=58946)
    channel.close()
    sys.exit()


class MyClient(Client):
  def __init__(self, intents: Intents) -> None:
    super().__init__(intents=intents)
    self.tree = CommandTree(self)
    self.tree.add_command(Remindcmd("remind"))
  async def on_ready(self):
    loop = asyncio.get_event_loop()
    loop.create_task(Notification())
    print("起動完了")
  async def setup_hook(self) -> None:
    print("同期開始")
    await self.tree.sync()
  async def on_close(self) -> None:
    print("終了します")

intents = Intents.default()
client = MyClient(intents=intents)

signal.signal(signal.SIGINT, signal_handler)

if token:
    client.run(token=token)
else:
    print("Token is not available.")
