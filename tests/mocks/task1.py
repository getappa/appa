import sys
import json

user = json.loads(sys.argv[1])
user.count = len(user["name"]) + user["age"]

sys.stdout.write(json.dumps(user))