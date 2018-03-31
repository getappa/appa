import sys
import json

json_str = json.dumps([
    { 'id': 1, 'name': 'gol', 'age': 11 },
    { 'id': 2, 'name': 'c3', 'age': 4 },
])

sys.stdout.write(json_str)