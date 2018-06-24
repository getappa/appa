import sys
import json

data = json.loads(sys.argv[1])
response = {}

for k, v in data.items():
    val = json.loads(v)
    if not val.get("age", 0) is 32:
        response[k] = v

sys.stdout.write("{}".format(json.dumps(response)))