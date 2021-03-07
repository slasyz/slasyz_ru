from typing import List

from slasyz_ru.db import Database
from slasyz_ru.models import Food


class CaloriesRepository:
    def __init__(self, database: Database):
        self._database = database

    def list_foods(self) -> List[Food]:
        res = []
        rows = self._database.select_rows('''
            SELECT id, name, proteins, fats, carbos, kcals, weight, price 
            FROM foods 
            ORDER BY name
        ''')
        for row in rows:
            res.append(Food(
                id=row[0],
                name=row[1],
                proteins=row[2],
                fats=row[3],
                carbos=row[4],
                kcals=row[5],
                weight=row[6],
                price=row[7],
            ))  # TODO: use kwargs

        return res

    def add_food(self, food: Food):
        row = self._database.select_row('''
            INSERT INTO foods(name, proteins, fats, carbos, kcals, weight, price) 
            VALUES(%(name)s, %(proteins)s, %(fats)s, %(carbos)s, %(kcals)s, %(weight)s, %(price)s)
            RETURNING id
        ''', food.dict())

        food.id = row[0]

    def edit_food(self, food: Food):
        self._database.select_row('''
            UPDATE foods
            SET name = %(name)s,
                proteins = %(proteins)s,
                fats = %(fats)s,
                carbos = %(carbos)s,
                kcals = %(kcals)s,
                weight = %(weight)s,
                price = %(price)s
            WHERE id = %(id)s
            RETURNING id
        ''', food.dict())

    def delete_food(self, id: int):
        self._database.select_row('''DELETE FROM foods WHERE id=%s RETURNING id''', (id,))
