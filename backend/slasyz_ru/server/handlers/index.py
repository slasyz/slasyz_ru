from logging import Logger

from flask import Flask, Blueprint, make_response, send_from_directory, g, request
from flask_sockets import Sockets

from slasyz_ru.db.sessions import SessionsRepository
from slasyz_ru.server.middleware import cookie


def register(app: Flask, sockets: Sockets, sessions_repository: SessionsRepository, logger: Logger):
    blueprint = Blueprint('index', 'index', url_prefix='/')

    @blueprint.route('/')
    def handler_index():
        resp = make_response(send_from_directory('../../templates', 'index.html'))

        if g.u == cookie.SELENIUM_COOKIE_ID:
            resp.set_cookie('s', '', expires=0)
        else:
            session_uuid = sessions_repository.create_session(g.u, request.remote_addr, request.user_agent.string)
            resp.set_cookie('s', session_uuid, samesite='Lax')

        return resp

    @blueprint.route('/app/')
    @blueprint.route('/app/<path:path>')
    def handler_app(path=''):
        resp = make_response(send_from_directory('../../templates', 'app.html'))

        if g.u == cookie.SELENIUM_COOKIE_ID:
            resp.set_cookie('s', '', expires=0)
        else:
            session_uuid = sessions_repository.create_session(g.u, request.remote_addr, request.user_agent.string)
            resp.set_cookie('s', session_uuid, samesite='Lax')

        return resp

    app.register_blueprint(blueprint)
