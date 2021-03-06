from flask import g, request, Response, url_for

from slasyz_ru.db.cookies import CookieRepository

COOKIE_NAME = 'u'
SELENIUM_COOKIE_ID = '77e6634e-428e-447f-ba5e-998f247995c8'


class CookieMiddleware:
    def __init__(self, repository: CookieRepository):
        self._repository = repository

    def cookie_before(self):
        g.set_u = False
        u = request.cookies.get(COOKIE_NAME)

        if u is not None:
            self._repository.upsert_cookie(u)
            g.u = u
        else:
            g.u = self._repository.create_cookie()
            g.set_u = True

    def cookie_after(self, resp: Response):
        if g.set_u:
            resp.set_cookie(COOKIE_NAME, g.u)

        return resp
