from dataclasses import dataclass
from typing import TypedDict

@dataclass
class ResponseSet:
    message: str
    yes: str
    no: str

# class ResponseSet(TypedDict):
#     message: str
#     yes: str
#     no: str

# response_sets: list[ResponseSet] = [
#   {
#    "message": "a",
#    "yes": "aaaa",
#    "no": "aaaa"
#   }
# ]

response_sets = [
  ResponseSet("『{title}』をやりましたか？まだ休んではだめですよ", "はい", "いいえ"),
  ResponseSet("『{title}』の進捗はどうですか？","いいかも","だめかも"),
  ResponseSet("お疲れ様です！『{title}』をやり遂げてくださいね","やり遂げた","あとで"),
  ResponseSet("休憩は大切ですが、『{title}』も頑張ってください！","がんばった","がんばる"),
  ResponseSet("これ『{title}』やりましたか？","やった","やってない"),
  ResponseSet("おい、『{title}』やれ","やったけど何","黙ろうね")
]

