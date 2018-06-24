import sys
import json

data = json.loads(sys.argv[1])
response = {}

for k, v in data.items():
    val = json.loads(v);
    val["extra"] = {
        "age": val["age"],
        "count": val["count"],
        "esc": val["esc"]
    }

    del val["age"]
    del val["count"]
    del val["esc"]

    response[k] = val

sys.stdout.write("{}".format(json.dumps(response))