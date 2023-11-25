import os
from discord import Intents, Client, Interaction, app_commands
from discord.app_commands import CommandTree
from dotenv import load_dotenv
load_dotenv()


@app_commands.command()
async def hello(interaction: Interaction):
    await interaction.response.send_message(f'Hello, {interaction.user.mention}')


class MyGroup(app_commands.Group):
  def __init__(self, name: str):
    super().__init__(name=name)
  @app_commands.command()
  async def hello(self, interaction: Interaction):
    await interaction.response.send_message(f'Hello, {interaction.user.mention}')


class MyClient(Client):
    def __init__(self, intents: Intents) -> None:
        super().__init__(intents=intents)
        self.tree = CommandTree(self)
        self.tree.add_command(hello)
        self.tree.add_command(MyGroup("group"))

    async def setup_hook(self) -> None:
        await self.tree.sync()

    async def on_ready(self):
        print(f"login: {self.user.name} [{self.user.id}]")
        print(self.get_guild(1047700283217686719))


intents = Intents.default()
client = MyClient(intents=intents)


client.run(os.getenv("TOKEN"))
