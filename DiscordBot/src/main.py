from remindcmd import Remindcmd
from discord import Intents, Client
from grpclib.client import Channel
from discord.app_commands import CommandTree
import os
from dotenv import load_dotenv

channel = Channel(host= "reminder", port=58946)

dotenv_path = '../.env'
load_dotenv(verbose=True, dotenv_path=dotenv_path)

token = os.getenv("TOKEN")

class MyClient(Client):
  def __init__(self, intents: Intents) -> None:
    super().__init__(intents=intents)
    self.tree = CommandTree(self)
    self.tree.add_command(Remindcmd("remind"))
  async def on_ready(self):
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
