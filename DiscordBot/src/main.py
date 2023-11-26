import discord
from remindcmd import Remindcmd
from discord import Intents, Client
from pb.dry import reminder
from grpclib.client import Channel
from discord.app_commands import CommandTree
import os
from dotenv import load_dotenv
from google.protobuf import empty_pb2


channel = Channel(host= "reminder", port=58946)

dotenv_path = '../.env'
load_dotenv(verbose=True, dotenv_path=dotenv_path)

token = os.getenv("TOKEN")

async def main():
  print("通知待機開始")
  channels = Channel(host= "reminder", port=58946)
  service = reminder.NotificationServiceStub(channels)
  async for response in service.push_notification(betterproto_lib_google_protobuf_empty=empty_pb2):
    print(response)
    Uid :int = 286014993772576770
    channel = client.get_channel(1064809411970867264)
    await channel.send(content= f"<@{Uid}>")


class MyClient(Client):
  def __init__(self, intents: Intents) -> None:
    super().__init__(intents=intents)
    self.tree = CommandTree(self)
    self.tree.add_command(Remindcmd("remind"))
  async def on_ready(self):
    await main()
    print("起動完了")
  async def setup_hook(self) -> None:
    print("同期開始")
    await self.tree.sync()
  async def on_close(self) -> None:
    print("終了します")
    channel.close()

intents = Intents.default()
client = MyClient(intents=intents)



if token:
    client.run(token=token)
else:
    print("Token is not available.")
