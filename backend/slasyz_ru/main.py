import logging
import os.path

from fastapi import FastAPI
from fastapi.templating import Jinja2Templates
import uvicorn

from slasyz_ru import config
from slasyz_ru.db import Database
from slasyz_ru.db.calories import CaloriesRepository
from slasyz_ru.handlers import index  # TODO: import all handlers dynamically
from slasyz_ru.handlers import calories

conf = config.load(os.path.join(os.path.dirname(__file__), '', '..', 'config.json'))


def create_app() -> FastAPI:
    logger = logging.getLogger('uvicorn')

    database = Database(
        conf['database']['host'],
        conf['database']['database'],
        conf['database']['user'],
        conf['database']['password'],
        logger
    )
    database.connect()

    app = FastAPI()
    templates = Jinja2Templates(directory=os.path.join(os.path.dirname(__file__), '', '..', 'templates'))

    # TODO: access log middleware

    calories_repository = CaloriesRepository(database)

    routes = [
        index.get_router(templates, logger),
        calories.get_router(calories_repository, logger),
    ]

    for route in routes:
        app.include_router(route)

    # TODO: handle errors

    return app


def __main__():
    app = create_app()

    uvicorn.run(app, host="0.0.0.0", port=8000)


if __name__ == '__main__':
    __main__()
