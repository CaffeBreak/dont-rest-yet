from __future__ import print_function
import discord
import datetime
import os
import re
from dotenv import load_dotenv
from os.path import join, dirname


from discord.ext import commands
from  discord import app_commands


import os.path

from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError

class CourseObj:
    pass

intents = discord.Intents.all()  # デフォルトのIntentsオブジェクトを生成
intents.typing = False  # typingを受け取らないように
intents.message_content = True  # メッセージコンテントの有効化
client = commands.Bot(
    command_prefix=commands.when_mentioned_or("!"),
    case_insensitive=True,
    intents=intents
)
# discordクライアントの準備、コマンドプレフィックス(先頭につける文字)を!に指定とメンションでも反応、上記3行のインテンツの読み込み
# 起動確認
dotenv_path = join(dirname(__name__), '.env')
load_dotenv(verbose=True, dotenv_path=dotenv_path)

print(intents.members)


@client.event
async def on_ready():
    # 起動後PC側にメッセージ送信
    print(datetime.datetime.now().time(),
          "on_ready/discordVer", discord.__version__)
    await client.change_presence(activity=discord.Activity(name="!pdfでPDFをjpgに", type=discord.ActivityType.playing))


@client.command()
async def list(ctx, *args):
    await ctx.defer()
    embed = discord.Embed(
            title ="受講授業一覧",
            color=0x0101010,
            description="",
            url=""
        )
    if(client.user != None):
        embed.set_author(
                name=client.user,
                icon_url=client.user.display_avatar.url
                )
    else:
        return "Cant_GET_Client_User"
    cB = getTaskList()
    if(cB != None):    
        for c in cB:
                embed.add_field(name=c.course['name'],value = "課題は"+str(len(c.tasks))+"件です",inline = False)
    await ctx.send(embed=embed)

# If modifying these scopes, delete the file token.json.
SCOPES = ['https://www.googleapis.com/auth/classroom.courses.readonly',
          'https://www.googleapis.com/auth/classroom.student-submissions.me.readonly']

def getActiveCoursesList():
    try:
        creds = authenticate_google_api()
        service = build('classroom', 'v1', credentials=creds)

        # Call the Classroom API
        results = service.courses().list().execute()
        courses = results.get('courses', [])

        if not courses:
            print('No courses found.')
            return
    except HttpError as error:
        print('An error occurred: %s' % error)
    activeCourses = []
    for c in courses: # type: ignore
        if (c['courseState'] == "ACTIVE"):
            activeCourses.append(c)
        
    return activeCourses

def authenticate_google_api():
    creds = None
    if os.path.exists('token.json'):
        creds = Credentials.from_authorized_user_file('token.json', SCOPES)
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file(
                'credentials.json', SCOPES)
            creds = flow.run_local_server(port=0)
        with open('token.json', 'w') as token:
            token.write(creds.to_json())
    return creds

def get_classroom_assignments(class_id, credentials):
    # 課題オブジェクトの中のなに使うかわからないのでとりあえず全取得、整形だけしてreturn
    service = build('classroom', 'v1', credentials=credentials)

    # クラスルームの課題を取得するAPI呼び出し
    try:
        course_works = service.courses().courseWork().list(courseId=class_id, courseWorkStates= "PUBLISHED").execute()
        return course_works['courseWork']
    except Exception as e:
        print(f"課題の取得中にエラーが発生しました: {e}")
        return None

def checkTaskDuw(Duedate):
#タスク確認ドワミンジュ川
    today = datetime.date.today()
    if(Duedate > today):
        return True
    return False

def getTaskList():
    credentials = authenticate_google_api() 
    classList = getActiveCoursesList()
    if (classList == None):
        return None
    
    # コース毎の課題を取れるといいね
    # 管理的にコースおｂｊと課題のクラスを作ってそれの配列を作った方がいい
    courseBox = []
    
    for c in classList:
        if(c['courseState'] == 'ACTIVE'):        
            tmp = CourseObj()
            tasks = []
            tmp.course = c
            res = get_classroom_assignments(c['id'], credentials)
            if (res != None):
                for work in res:
                    if('dueDate' in work and checkTaskDuw(datetime.date(work['dueDate']['year'],work['dueDate']['month'], work['dueDate']['day']))):
                        tasks.append(work)
            tmp.tasks   = tasks
            courseBox.append(tmp)

    return courseBox


client.run(os.getenv('DISCORD_TOKEN')) # type: ignore
