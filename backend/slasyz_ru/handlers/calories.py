from logging import Logger
from typing import List

from fastapi import APIRouter, Request, Body

from slasyz_ru.db.calories import CaloriesRepository
from slasyz_ru.models import Food


def get_router(repo: CaloriesRepository, logger: Logger) -> APIRouter:
    router = APIRouter(
        prefix='/api/foods',  # TODO: create separate /api router
        responses={404: {'description': 'Not found'}},
    )

    @router.get('/list', response_model=List[Food])
    async def list_foods(request: Request):
        return repo.list_foods()

    @router.post('/add', response_model=Food)
    async def add_food(request: Request, food: Food = Body(..., embed=True)):
        repo.add_food(food)
        return food

    @router.post('/edit', response_model=Food)
    async def edit_food(request: Request, food: Food = Body(..., embed=True)):
        repo.edit_food(food)
        return food

    @router.delete('/delete')
    async def delete_food(request: Request, id: int = Body(..., embed=True)):
        repo.delete_food(id)  # TODO: not exists exception
        return {}

    return router
