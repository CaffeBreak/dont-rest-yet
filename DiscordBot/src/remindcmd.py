from discord import app_commands, Interaction
import calendar
import discord
import asyncio
from grpclib.client import Channel
from pb.dry import reminder
from discord.ui import Select, View
from datetime import datetime, timezone
from discord import Embed

class SelectView(View):
  @discord.ui.select(
         cls=Select,
         placeholder="リマインドを選択してください",
         options=[],
  )
  async def selectMenu(self, interaction: Interaction, select: Select):
    selected_value = ''.join(select.values)
    request = reminder.DeleteTaskRequest(
      id=str(selected_value)
    )
    print(select)
    print(selected_value)
    channel = Channel(host= "reminder", port=58946)
    service = reminder.TaskServiceStub(channel)
    response = await service.delete_task(request)
    print(response)
    await interaction.response.send_message(content=f"<@{interaction.user.id}>選択されたリマインド{response.title}を削除しました")

class Remindcmd(app_commands.Group):
  def __init__(self, name: str):
    super().__init__(name = name)
    
  @app_commands.command(name="add", description="リマインドを行います")
  async def add(self, interaction: Interaction, main: str, days: str, time: str):
    """
    
    main : str
        リマインドしたい内容 
        
    days : str
        リマインドしたい日付(10/31)
    
    time : str
        リマインドする時間(18:00)
    """
    try:
        month, day = map(int, days.split('/'))
        hour, minute = map(int, time.split(':'))
    except ValueError:
        if '/' not in days:
            await interaction.response.send_message(content=f"<@{interaction.user.id}>日付の形式が正しくありません。月/日の形式で入力してください")
            return
        if ':' not in time:
            await interaction.response.send_message(content=f"<@{interaction.user.id}>時間の形式が正しくありません。時間:分の形式で入力してください")
            return
        return

    if not (1 <= month <= 12):
        await interaction.response.send_message(content=f"<@{interaction.user.id}>月の値が範囲外です。1から12の間で入力してください")
        return

    # 月ごとの最大の日付を取得
    max_day_in_month = calendar.monthrange(year=datetime.now().year, month=month)[1]
    
    if not (1 <= day <= max_day_in_month):
        await interaction.response.send_message(content=f"<@{interaction.user.id}>日の値が範囲外です。{month}月は1日から{max_day_in_month}日の間で入力してください")
        return

    if not (0 <= hour <= 23):
        await interaction.response.send_message(content=f"<@{interaction.user.id}>時間の値が範囲外です。0から23の間で入力してください")
        return

    if not (0 <= minute <= 59):
        await interaction.response.send_message(content=f"<@{interaction.user.id}>分の値が範囲外です。0から59の間で入力してください")
        return


    year = datetime.now().year
    now = datetime.now(timezone.utc)
    if datetime(year, month, day, hour, minute, 0, tzinfo=timezone.utc) < now:
      year += 1  # 今の時刻より前ならば、翌年にする

    Uid = interaction.user.id
    print(Uid)
    print(type(Uid))
    
    request = reminder.CreateTaskRequest(
      title=main,
      remind_at=datetime(year, month, day, hour, minute, 0, tzinfo=timezone.utc),
      who=str(Uid)
    )
    channel = Channel(host= "reminder", port=58946)
    service = reminder.TaskServiceStub(channel)
    response = await service.create_task(request)
    print(response)

      
    await interaction.response.send_message(content=f"<@{interaction.user.id}> {days}-{time}に{main}をリマインドします。 ")
    
  @app_commands.command(name="list", description="リマインドのリストを表示します")
  async def list(self, interaction: Interaction):
    Uid = interaction.user.id
    request = reminder.ListTaskRequest(
      who= str(Uid)
    )
    channel = Channel(host= "reminder", port=58946)
    service = reminder.TaskServiceStub(channel)
    response = await service.list_task(request)
    
    tasks = response.tasks
    if not tasks:
        await interaction.response.send_message("リマインドはありません。")
        return
    
    embed = Embed(title="リマインドリスト", color=0x00ff00)
    
    for task in tasks:
      embed.add_field(
          name=f"{task.title}",
          value=f"日時: {task.remind_at.strftime('%Y-%m-%d %H:%M')}",
          inline=False
      )

    # メッセージにEmbedを追加して送信
    await interaction.response.send_message(content=f"<@{interaction.user.id}>",embed=embed)
    
    
    print(response.tasks)
    
  @app_commands.command(name="delete", description="リマインドの削除を行います")
  async def delete(self, interaction: Interaction,):
    Uid = interaction.user.id
    request = reminder.ListTaskRequest(
      who= str(Uid)
    )
    channel = Channel(host= "reminder", port=58946)
    service = reminder.TaskServiceStub(channel)
    response = await service.list_task(request)
    
    tasks = response.tasks
    if not tasks:
        await interaction.response.send_message(content=f"<@{interaction.user.id}>リマインドはありません。")
        return
    options = []
      
        # タスクごとにdiscord.SelectOptionオブジェクトを生成し、optionsに追加
    for task in tasks:
      formatted_datetime = task.remind_at.strftime('%Y-%m-%d %H:%M')
      label_with_datetime = f"{task.title} - {formatted_datetime}"
      option = discord.SelectOption(label=label_with_datetime, value=task.id)
      options.append(option)

    view = SelectView()
    # selectMenuのoptionsを更新
    view.selectMenu.options = options
    await interaction.response.send_message("どのリマインドを削除しますか？", view=view)
    await asyncio.sleep(20)
    await interaction.delete_original_response()




    
