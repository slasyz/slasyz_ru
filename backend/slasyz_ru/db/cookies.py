from slasyz_ru import db


class CookieRepository:
    def __init__(self, database: db.Database):
        self._database = database

    def upsert_cookie(self, cookie_uuid: str):
        self._database.execute('INSERT INTO cookies(uuid) VALUES(%s) ON CONFLICT DO NOTHING', (cookie_uuid,))

    def create_cookie(self) -> str:
        row = self._database.select_row('INSERT INTO cookies(uuid) VALUES(DEFAULT) RETURNING uuid')
        return row[0]
