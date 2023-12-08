from datetime import timedelta, timezone
from pb.dry import reminder
from datetime import datetime, timezone
from pb.dry.reminder import Task
from discord import Interaction
from grpclib.client import Channel
JST = timezone(timedelta(hours=+9), "JST")

Grpcclient = "Discord"
  
async def User_check (interaction:Interaction,Uid,):
  if not interaction.message:
      return 
  message_id = interaction.message.id
  interactionid = interaction.user.id
  if interactionid != Uid:
    await interaction.followup.edit_message(message_id=message_id,content= f"{interaction.message.content.split('\n')[0]}\n<@{interactionid}>いたずらはだめですよ！！")
    return True
  return False 


async def Update_task (interaction:Interaction,response: Task, dley):
  channel = Channel(host="reminder", port=58946)
  service = reminder.TaskServiceStub(channel)
  if not interaction.message:
    return 
  message_id = interaction.message.id
  request = reminder.UpdateTaskRequest(
    id=response.id,
    title=response.title,
    remind_at=datetime.now(JST) + dley
  )
  response = await service.update_task(request)
  await interaction.followup.edit_message(message_id=message_id,content=f"更新しました。{response.remind_at.astimezone(JST).strftime('%Y-%m-%d %H:%M')}に再度通知します", view=None)

async def Get_task (task_id):
  channel = Channel(host="reminder", port=58946)
  service = reminder.TaskServiceStub(channel)
  request = reminder.GetTaskRequest(
        id= task_id
  )
  response = await service.get_task(request)
  return response 
