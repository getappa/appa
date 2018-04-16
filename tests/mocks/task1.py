import sys
import json

class AppaCommand():
    def __init__(self):
        data = json.loads(sys.argv[1])

        if isinstance(data, list):
            data = data[0]

        self.data = data

    def send(self, val):
        sys.stdout.write("{}".format(val))

    @staticmethod
    def run(command):
        c = command()
        val = c.get_val()
        c.send(val)



class Task(AppaCommand):
    def get_val(self):
        return len(self.data["name"]) + self.data["age"]

AppaCommand.run(Task)