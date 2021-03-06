from flask import Flask, Blueprint, render_template, url_for, send_file

from slasyz_ru.db.sessions import SessionsRepository


def login_required():
    return  # TODO: login required


def register(app: Flask, sessions_repository: SessionsRepository):
    blueprint = Blueprint('admin', 'admin', url_prefix='/admin')

    @blueprint.route('/')
    def handler_index():
        return 'yes'

    @blueprint.route('/sessions')
    @blueprint.route('/sessions/<int:page>')
    def handler_sessions(page: int = 1):
        sessions = sessions_repository.get_sessions(page)

        page_prev_link = None
        if page > 1:
            page_prev_link = url_for('admin.handler_sessions', page=page-1)

        return render_template('admin/sessions.html', **{
            'sessions': sessions,
            'page': page,
            'page_prev_link': page_prev_link,
            'page_next_link': url_for('admin.handler_sessions', page=page+1),
        })

    blueprint.before_request(login_required)

    app.register_blueprint(blueprint)
