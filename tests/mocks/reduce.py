import sys
import json

data = json.loads(sys.argv[1])
response = {}

for (k, v in data):
    if v["age"] is not 32:
        val[k] = v

sys.stdout.write("{}".format(response))