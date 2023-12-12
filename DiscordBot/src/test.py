import json

# 書き込み
channel_id = 1064809411970867264
with open("config.json", "w") as file:
    json.dump({"channel_id": channel_id}, file)

old_nickname = "朝食勇気"
with open("nikiname.json", "w") as file:
    json.dump({"old_nickname": old_nickname}, file)
