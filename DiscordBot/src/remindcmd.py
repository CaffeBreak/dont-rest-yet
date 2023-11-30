from discord import Color, SelectOption, app_commands, Interaction
import calendar
import discord
from grpclib.client import Channel
from const import JST
from pb.dry import reminder
from discord.ui import Select, View
from datetime import datetime, timedelta, timezone
from discord import Embed
from typing import Any, Callable, Coroutine, Optional, Union

from view import DeletePaginationView, PaginationView
    
class SelectView(View):
  @discord.ui.select(
         cls=Select,
         placeholder="リマインドを選択してください",
         options=[],
  )
  async def selectMenu(self, interaction: Interaction, select: Select):
    await interaction.response.defer(ephemeral=True)
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
    
    await interaction.followup.send(content=f"選択されたリマインド{response.title}を削除しました",ephemeral=True)

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
    await interaction.response.defer(ephemeral=True)
    try:
        month, day = map(int, days.split('/'))
        hour, minute = map(int, time.split(':'))
    except ValueError:
        if '/' not in days:
            await interaction.followup.send(content=f"日付の形式が正しくありません。月/日の形式で入力してください")
            return
        if ':' not in time:
            await interaction.followup.send(content=f"時間の形式が正しくありません。時間:分の形式で入力してください")
            return
        return

    if not (1 <= month <= 12):
        await interaction.followup.send(content=f"月の値が範囲外です。1から12の間で入力してください")
        return

    # 月ごとの最大の日付を取得
    max_day_in_month = calendar.monthrange(year=datetime.now().year, month=month)[1]
    
    if not (1 <= day <= max_day_in_month):
        await interaction.followup.send(content=f"日の値が範囲外です。{month}月は1日から{max_day_in_month}日の間で入力してください")
        return

    if not (0 <= hour <= 23):
        await interaction.followup.send(content=f"時間の値が範囲外です。0から23の間で入力してください")
        return

    if not (0 <= minute <= 59):
        await interaction.followup.send(content=f"分の値が範囲外です。0から59の間で入力してください")
        return


    now = datetime.now(timezone.utc)
    year = now.year
    print(f"比較する時間は{now}")
    # print(f"タスクを作成しようとしている日時は{year}-{month}-{day}-{hour}-{minute}")
    task_time = datetime(year, month, day, hour, minute, 0, tzinfo=JST)
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
      remind_at=task_time.astimezone(timezone.utc),
      who=str(Uid)
    )
    channel = Channel(host= "reminder", port=58946)
    service = reminder.TaskServiceStub(channel)
    response = await service.create_task(request)
    print("タスク作成")
    print(response)
    await interaction.followup.send(content=f" {response.remind_at.astimezone(JST)}に{response.title}をリマインドします。")#ephemeral=True→「これらはあなただけに表示されています」
    
  @app_commands.command(name="list", description="リマインドのリストを表示します")
  async def list(self, interaction: Interaction):
    uid = interaction.user.id
    request = reminder.ListTaskRequest(
      who=str(uid)
    )
    channel = Channel(host="reminder", port=58946)
    service = reminder.TaskServiceStub(channel)
    response = await service.list_task(request)

    tasks = response.tasks
    if not tasks:
      await interaction.response.send_message("リマインドはありません。")
      return
      
    view: PaginationView[reminder.Task] = PaginationView(tasks, gen_embed, "リマインドリスト", 0x00ff00)

    # メッセージに Embed を追加して送信
    await interaction.response.send_message(content=f"", embed=view.get_init_embed(), view=view)
    
    
    print(response.tasks)
    

  @app_commands.command(name="delete", description="リマインドの削除を行います")
  async def delete(self, interaction: Interaction):
    uid = interaction.user.id
    request = reminder.ListTaskRequest(
      who= str(uid)
    )
    channel = Channel(host="reminder", port=58946)
    service = reminder.TaskServiceStub(channel)
    response = await service.list_task(request)
    tasks = response.tasks
    
    if not tasks:
      await interaction.response.send_message(content=f"リマインドはありません。")
      
      return
    
    gen_options: Callable[[list[reminder.Task]], list[SelectOption]] = lambda tasks: [SelectOption(label=f"{i + 1}. {task.title}", value=task.id) for i, task in enumerate(tasks)]
    delete_task: Callable[[str], Coroutine[Any, Any, reminder.Task]] = lambda id: service.delete_task(reminder.DeleteTaskRequest(id))

    view = DeletePaginationView(tasks, gen_embed, gen_options, delete_task, "リマインドリスト", 0x00ff00)
    # optionsを更新
    view.delete_target.options = gen_options(tasks[0:10])
    await interaction.response.send_message(content="削除する対象の番号を選んでください", view=view, embed=view.get_init_embed())
    # await asyncio.sleep(20)
    # await interaction.delete_original_response()
  
def gen_embed(tasks: list[reminder.Task], index: int, page_max: int, title: Optional[str], color: Optional[Union[int, Color]]) -> Embed:
    embed = Embed(title=f"{title} - Page[{index + 1}/{page_max + 1}]", color=color)
    embed.add_field(
      name="",
      value=f"\n".join([f"`{str(i + 1).rjust(2, " ")}. {task.title}` - {task.remind_at.strftime('%Y-%m-%d %H:%M')}" for i, task in enumerate(tasks)]),
      inline=False
    )
    
    return embed
