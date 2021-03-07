import json


def load(filepath):
    with open(filepath) as f:
        return json.load(f)
