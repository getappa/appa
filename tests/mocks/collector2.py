import sys
import json
from time import sleep

data = [
    { 'id': 1, 'name': 'gol', 'age': 11 },
    { 'id': 2, 'name': 'c3', 'age': 4 },
    { 'id': 3, 'name': 'fox', 'age': 6 },
]

for v in data:
    json_str = json.dumps([v])
    sys.stdout.write("{}\n".format(json_str))
    sys.stdout.flush()
    sleep(1)