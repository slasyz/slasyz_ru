from decimal import Decimal
from typing import Optional

from pydantic import BaseModel


class Food(BaseModel):
    id: Optional[int]
    name: str

    proteins: Decimal
    fats: Decimal
    carbos: Decimal
    kcals: Decimal

    weight: Optional[int]
    price: Optional[int]
