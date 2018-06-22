import sys
import json
from time import sleep

data = [
    { 'id': 1, 'name': 'gol', 'age': 11 },
    { 'name': 'pedro', 'age': 32 },
    { 'id': 2, 'name': 'c3', 'age': 4 },
    { 'name': 'carol', 'age': 23 },
    { 'id': 3, 'name': 'fox', 'age': 6 },
]

for v in data:
    json_str = json.dumps([v])
    tag = 'car' if v.has_key("id") else 'user'
    sys.stdout.write("!AppaTag({}){}\n".format(tag, json_str))
    sys.stdout.flush()
    sleep(1)