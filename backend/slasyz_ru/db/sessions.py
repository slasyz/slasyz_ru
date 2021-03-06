from dataclasses import dataclass
from datetime import datetime
from typing import List

from flask import url_for

from slasyz_ru.db import Database


@dataclass
class Session:
    uuid: str
    cookie_id: str
    size: str
    clickpath_size: int
    ip: str
    useragent: str
    created: datetime
    cookie_uuid: str

    def url(self) -> str:
        return ''
        # return url_for('admin.handler_clickpath', session_uuid=self.uuid)


@dataclass
class Clickpath:
    width: int
    height: int
    path: str


class InvalidSizeException(Exception):
    pass


class SessionsRepository:
    def __init__(self, database: Database):
        self._database = database

    def get_sessions(self, page: int) -> List[Session]:
        limit = 50
        offset = limit * (page-1)

        res = []
        rows = self._database.select_rows(
            '''
            SELECT s.uuid, cookie_id, size, length(clickpath), ip, useragent, created, c.uuid
            FROM sessions s
            JOIN cookies c on s.cookie_id = c.id
            ORDER BY s.id DESC
            OFFSET %s
            LIMIT %s
            ''',
            (offset, limit)
        )
        for row in rows:
            res.append(Session(*row))

        return res

    def create_session(self, cookie_uuid: str, ip: str, useragent: str) -> str:
        row = self._database.select_row(
            '''
            INSERT INTO sessions(cookie_id, ip, useragent)
            VALUES((SELECT id FROM cookies WHERE uuid=%s), %s, %s)
            RETURNING uuid
            ''',
            (cookie_uuid, ip, useragent),
        )
        return row[0]

    def get_clickpath(self, session_uuid: str) -> Clickpath:
        row = self._database.select_row('SELECT size, clickpath FROM sessions WHERE uuid = %s', (session_uuid,))

        size_arr = row[0].split('x')
        if len(size_arr) != 2:
            raise InvalidSizeException(row[0])

        width, height = int(size_arr[0]), int(size_arr[1])
        clickpath = row[1]

        return Clickpath(width, height, clickpath)

    def set_session_size(self, session_uuid: str, width: int, height: int):
        self._database.execute(
            '''
            UPDATE sessions
            SET size = %s
            WHERE uuid = %s
            ''',
            ('{}x{}'.format(width, height), session_uuid)
        )

    def append_session_clickpath(self, session_uuid: str, clickpath_chunk: str):
        self._database.execute(
            '''
            UPDATE sessions
            SET clickpath = concat(clickpath, %s)
            WHERE uuid = %s
            ''',
            (clickpath_chunk, session_uuid)
        )
