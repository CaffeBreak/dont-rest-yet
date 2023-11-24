import grpc
from discord import Client, app_commands, Interaction
from pb import discorbot_pb2_grpc
from pb import discorbot_pb2
from datetime import datetime, timezone
from google.protobuf.timestamp_pb2 import Timestamp

def convert_to_timestamp(year, month, day, hour=0, minute=0, second=0):
    # datetimeオブジェクトを作成
    dt = datetime(year, month, day, hour, minute, second, tzinfo=timezone.utc)

    # datetimeオブジェクトをtimestampに変換
    timestamp = int(dt.timestamp())

    # Timestampメッセージを作成
    timestamp_message = Timestamp(seconds=timestamp)

    return timestamp_message


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
    month, day = map(int, days.split('/'))
    hour, minute = map(int, time.split(':'))
    year = datetime.now().year
    Uid = interaction.user.id
    print(Uid)
    result = convert_to_timestamp(year, month, day, hour, minute, 0)
    
    print(type(result))
    print(type(Uid))
    
    request = discorbot_pb2.CreateTaskRequest(
      title=main,
      remindAt=result,
      who=str(Uid)
    )
    with grpc.insecure_channel('reminder:58946') as channel:
        stub = discorbot_pb2_grpc.TaskServiceStub(channel)
        response = stub.CreateTask(request)
    print(response)

      
    await interaction.response.send_message(f"Hi, {main} {day} {time} ")
    

