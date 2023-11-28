from discord import app_commands, Interaction
import calendar
import discord
import asyncio
from grpclib.client import Channel
from pb.dry import reminder
from discord.ui import Select, View, Button
from datetime import datetime, timezone
from discord import Embed
from typing import List

    # view = SelectView()
    # # selectMenuのoptionsを更新
    # view.selectMenu.options = options
    
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


# class SelectViewpage(View):
#     def __init__(self, initial_options: List[discord.SelectOption]) -> None:
#         super().__init__()
#         self.options = initial_options
#         self.page = 0
        
    
#     @discord.ui.button(label="次のページ", style=discord.ButtonStyle.success)
#     async def next_page(self, button: discord.Button, interaction: discord.Interaction):
#         self.page += 1
#         await self.refresh_options(interaction)

#     @discord.ui.button(label="前のページ", style=discord.ButtonStyle.success)
#     async def prev_page(self, button: discord.Button, interaction: discord.Interaction):
#         if self.page > 0:
#             self.page -= 1
#         await self.refresh_options(interaction)
    
#     async def refresh_options(self, interaction: discord.Interaction):
#       start_idx = self.page * 25
#       end_idx = start_idx + 25
#       current_options = self.options[start_idx:end_idx]

#         # ページの始まりに戻るボタンを追加
#       if self.page > 0:
#         current_options.insert(0, discord.SelectOption(label="前のページ", value="prev_page"))

#         # ページの最後に次のページボタンを追加
#       if end_idx < len(self.options):
#         current_options.append(discord.SelectOption(label="次のページ", value="next_page"))

#         # optionsを更新
#       self.options = current_options
#       view = SelectView()
#       view.selectMenu.options = current_options
#         # ユーザーへのメッセージを更新
#       await interaction.edit_original_response(view=view)




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


    now = datetime.now(timezone.utc)
    year = now.year
    print(f"比較する時間は{now}")
    # print(f"タスクを作成しようとしている日時は{year}-{month}-{day}-{hour}-{minute}")
    task_time = datetime(year, month, day, hour, minute, 0, tzinfo=timezone.utc)
    print(f"{task_time}")
    if task_time < now:
      print("作成するタスクを翌年にします")
      year += 1  # 今の時刻より前ならば、翌年にする
      
    print(f"タスクを作成する日時は{year}-{month}-{day}-{hour}-{minute}")

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
    print("タスク作成")
    print(response)
    await interaction.response.send_message(content=f"<@{interaction.user.id}> {days}-{time}に{main}をリマインドします。 ")
    
  @app_commands.command(name="list", description="リマインドのリストを表示します")
  async def list(self, interaction: Interaction, page: int):
    """
    
    page  : int
        表示するリストのページ数を指定します。 
    """
    Uid = interaction.user.id
    request = reminder.ListTaskRequest(
      who=str(Uid)
    )
    channel = Channel(host="reminder", port=58946)
    service = reminder.TaskServiceStub(channel)
    response = await service.list_task(request)

    tasks = response.tasks
    if not tasks:
      await interaction.response.send_message("リマインドはありません。")
      return

        # 1ページに表示するタスク数
    tasks_per_page = 25

        # ページ数を計算し、表示すべきタスクの範囲を取得
    total_pages = (len(tasks) + tasks_per_page - 1) // tasks_per_page
    if page < 1 or page > total_pages:
        await interaction.response.send_message(f"無効なページ番号です。1から{total_pages}までの範囲で指定してください。")
        return

    start_idx = (page - 1) * tasks_per_page
    end_idx = start_idx + tasks_per_page
    current_tasks = tasks[start_idx:end_idx]

    embed = Embed(title=f"リマインドリスト - ページ {page}/{total_pages}", color=0x00ff00)

    for task in current_tasks:
      embed.add_field(
        name=f"{task.title}",
        value=f"日時: {task.remind_at.strftime('%Y-%m-%d %H:%M')}",
        inline=False
      )

        # メッセージに Embed を追加して送信
    await interaction.response.send_message(content=f"<@{interaction.user.id}>", embed=embed)
    
    
    print(response.tasks)
    
  @app_commands.command(name="delete", description="リマインドの削除を行います")
  async def delete(self, interaction: Interaction, page: int):
    """
        page  : int
        削除するタスクを選択するタスクリストのページ数を指定します。 
    """
    Uid = interaction.user.id
    request = reminder.ListTaskRequest(
      who= str(Uid)
    )
    channel = Channel(host= "reminder", port=58946)
    service = reminder.TaskServiceStub(channel)
    response = await service.list_task(request)
    tasks = response.tasks
    tasks_per_page = 25
    
    total_pages = (len(tasks) + tasks_per_page - 1) // tasks_per_page
    if page < 1 or page > total_pages:
        await interaction.response.send_message(f"無効なページ番号です。1から{total_pages}までの範囲で指定してください。")
        return
    
    if not tasks:
        await interaction.response.send_message(content=f"<@{interaction.user.id}>リマインドはありません。")
        return
    options = []
    
    start_idx = (page - 1) * tasks_per_page
    end_idx = start_idx + tasks_per_page
    current_tasks = tasks[start_idx:end_idx]
      
        # タスクごとにdiscord.SelectOptionオブジェクトを生成し、optionsに追加
    for task in current_tasks:
      formatted_datetime = task.remind_at.strftime('%Y-%m-%d %H:%M')
      label_with_datetime = f"{task.title} - {formatted_datetime}"
      option = discord.SelectOption(label=label_with_datetime, value=task.id)
      options.append(option)

    view = SelectView()
    # selectMenuのoptionsを更新
    view.selectMenu.options = options
    # view = SelectViewpage(initial_options=options)
    await interaction.response.send_message(view=view)
    await asyncio.sleep(20)
    await interaction.delete_original_response()




    
