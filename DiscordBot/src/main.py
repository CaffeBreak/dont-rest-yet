from operator import ne
import signal
import sys
import discord
from schedule import idle_seconds
from remindcmd import Remindcmd
from discord import Emoji, Intents, Client, abc
from pb.dry import reminder
from pb.dry.reminder import UserIdentifier
from grpclib.client import Channel
from discord.app_commands import CommandTree
import os
from const import JST,Grpcclient,Update_task,User_check,Get_task
import random
import asyncio
from dotenv import load_dotenv
import json
from google.protobuf.empty_pb2 import Empty
from typing import Any, Callable,Coroutine
from datetime import datetime, timedelta
from view import bottonView, updattask
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
  async def on_interaction(self, inter:discord.Interaction):
    try:  
      if hasattr(inter, "data"):
        if inter.data['component_type']: # type: ignore
            await on_button_click(inter)
    except KeyError:
        pass


async def on_button_click(interaction:discord.Interaction):
  custom_id = interaction.data.get("custom_id")  # type: ignore
  if custom_id and '-' in custom_id:
    handle_id, task_id = custom_id.split('-')
  else:
    return
  await interaction.response.defer()
  if hasattr(interaction, "data"):
    if not interaction.message:
      return 
    message_id = interaction.message.id
    name = interaction.user.display_name
    response = await Get_task(task_id)
    Uid = int(response.who.identifier.strip("'"))
    if handle_id == "yes":
      if await User_check(interaction,Uid):
        return
      await interaction.followup.edit_message(message_id=message_id,content=f"{name}は『{response.title}』を完了しました！<:igyou:1019981052565000252><:erai:1045259439785123911>", view=None) 
    if handle_id == "no":
      if await User_check(interaction,Uid):
        return
      view = updattask(response.id)
      await interaction.followup.edit_message(message_id=message_id,content= f"<@{Uid}>いつ再リマインドしますか？",view=view)
    
    if handle_id =="day":
      if await User_check(interaction,Uid):
        return
      await Update_task(interaction,response,timedelta(days=1))
    if handle_id =="halfday":
      if await User_check(interaction,Uid):
        return
      await Update_task(interaction,response,timedelta(hours=12))
    if handle_id =="minutes":
      if await User_check(interaction,Uid):
        return
      await Update_task(interaction,response,timedelta(minutes=10))

async def Notification(channel_id):
  channel = client.get_channel(channel_id)
  if not isinstance(channel, abc.Messageable):
    print("通知の送信先がメッセージを送信可能なチャンネルではありません")
    
    sys.exit(1)
  
  channels = Channel(host= "reminder", port=58946)
  service1 = reminder.NotificationServiceStub(channels)
  service2 = reminder.TaskServiceStub(channels)
  
  print("通知待機開始")

  async for response in service1.push_notification(reminder.PushNotificationRequest(client=Grpcclient)):
    print("タスク受け取り中")
    print(response.who.identifier)
    Uid = int(response.who.identifier.strip("'"))
    selected = random.choice(response_sets)
    message = selected.message.format(title = response.title)
    view = bottonView(response.id,selected.yes,selected.no)
    await channel.send(content= f"<@{Uid}>{message}",view=view)

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
