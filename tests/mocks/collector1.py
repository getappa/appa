import sys
import json

json_str = json.dumps([
    { 'name': 'guilherme', 'age': 23 },
    { 'name': 'heydineia', 'age': 32 },
])

sys.stdout.write("!AppaTag(_){}\n".format(json_str))