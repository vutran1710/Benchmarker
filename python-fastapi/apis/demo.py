""" Demo API
"""
from fastapi import APIRouter

router = APIRouter()


@router.get("/plus")
async def get_best_posts(x: int, y: int):
    """ Get some thig
    """
    # NOTE: RedisClient is a singleton class,
    # will return the early-inited instance
    return {'total': x + y}
