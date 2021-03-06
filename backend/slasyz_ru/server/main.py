import logging
import os.path

from flask import Flask
from flask_sockets import Sockets

from slasyz_ru import config
from slasyz_ru.db import Database
from slasyz_ru.db.cookies import CookieRepository
from slasyz_ru.db.sessions import SessionsRepository
from slasyz_ru.server.handlers import admin, index  # TODO: import all handlers dynamically
from slasyz_ru.server.middleware.cookie import CookieMiddleware

conf = config.load(os.path.join(os.path.dirname(__file__), '..', '..', 'config.json'))


def create_app() -> Flask:
    logger = logging.getLogger(__name__)

    database = Database(
        conf['database']['host'],
        conf['database']['database'],
        conf['database']['user'],
        conf['database']['password'],
        logger
    )
    database.connect()

    app = Flask(__name__, static_url_path='', template_folder=os.path.join(os.path.dirname(__file__), '..', '..', 'templates'))
    sockets = Sockets(app)

    cookie_repository = CookieRepository(database)
    sessions_repository = SessionsRepository(database)

    cookie_middleware = CookieMiddleware(cookie_repository)
    app.before_request(cookie_middleware.cookie_before)
    app.after_request(cookie_middleware.cookie_after)

    index.register(app, sockets, sessions_repository, logger)
    admin.register(app, sessions_repository)

    # TODO: handle errors

    if "gunicorn" in os.environ.get("SERVER_SOFTWARE", ""):
        gunicorn_logger = logging.getLogger('gunicorn.error')
        app.logger.handlers.extend(gunicorn_logger.handlers)
        app.logger.handlers = gunicorn_logger.handlers
        app.logger.setLevel(gunicorn_logger.level)

    return app


def __main__():
    app = create_app()

    host, _, port = conf['server']['bind'].partition(":")
    app.run(host=host, port=port)  # TODO: use flask_sockets.worker somehow


if __name__ == '__main__':
    __main__()
