import sys
import json

class AppaCommand():
    def __init__(self):
        self.data = json.loads(sys.argv[1])

    def send(self, val):
        sys.stdout.write("{}".format(val))

    @staticmethod
    def run(command):
        c = command()
        val = c.get_val()
        c.send(val)



class Task(AppaCommand):
    def get_val(self):
        self.data["preprocess"] = "--pre={}".format(self.data["name"])
        return json.dumps(self.data)

AppaCommand.run(Task)