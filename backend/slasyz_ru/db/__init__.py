from logging import Logger

import psycopg2


class Database:
    def __init__(self, host, database, user, password, logger: Logger):
        self._host = host
        self._database = database
        self._user = user
        self._password = password

        self._logger: Logger = logger

        self._conn = None

    def connect(self):
        self._conn = psycopg2.connect(host=self._host, database=self._database, user=self._user, password=self._password)
        self._logger.info("connected to database")

    def select_rows(self, query, params):  # TODO: type hint
        with self._conn, self._conn.cursor() as cur:
            cur.execute(query, params)
            for record in cur:
                yield record

    def select_row(self, query, params=None):  # TODO: type hint
        with self._conn, self._conn.cursor() as cur:
            cur.execute(query, params)
            return cur.fetchone()

    def execute(self, query, params=None):
        with self._conn, self._conn.cursor() as cur:
            cur.execute(query, params)
