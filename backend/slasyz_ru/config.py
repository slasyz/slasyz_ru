import json
import os
from importlib.machinery import SourceFileLoader


def load(filepath):
    with open(filepath) as f:
        return json.load(f)


if isinstance(__loader__, SourceFileLoader):
    # Doing this if it is imported as `importlib.import_module(module_name)`.
    # It means it's most probably used as gunicorn config.
    _conf = load(os.path.join(os.path.dirname(__file__), '../config.json'))

    bind = _conf['server']['bind']
