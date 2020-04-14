""" Demo API
"""
from fastapi import APIRouter

router = APIRouter()


@router.get("/plus")
async def get_best_posts(x: int, y: int):
    return {'total': x + y}
