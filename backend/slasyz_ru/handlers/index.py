from logging import Logger

from fastapi import APIRouter, Request
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates


def get_router(templates: Jinja2Templates, logger: Logger) -> APIRouter:
    router = APIRouter(
        prefix='',
        responses={404: {'description': 'Not found'}},
    )

    @router.get('/', response_class=HTMLResponse)
    async def index(request: Request):
        return templates.TemplateResponse('index.html', {'request': request})

    @router.get('/app/', response_class=HTMLResponse)
    @router.get('/app/{path}/', response_class=HTMLResponse)
    async def app(request: Request, path: str = ''):
        return templates.TemplateResponse('app.html', {'request': request})

    return router
