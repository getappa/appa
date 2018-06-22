import sys
import json

data = json.loads(sys.argv[1])

for (d in data):
    d["extra"] = {
        "age": data["age"],
        "count": data["count"],
        "esc": data["esc"]
    }

    del data["age"]
    del data["count"]
    del data["esc"]

sys.stdout.write("{}".format(data))