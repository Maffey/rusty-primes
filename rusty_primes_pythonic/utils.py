from functools import wraps
from time import time
from typing import Callable


def time_function(function: Callable):
    @wraps(function)
    def wrap(*args, **kwargs):
        time_start = time()
        result = function(*args, **kwargs)
        time_end = time()
        print(f'Function: {function.__name__!r} took: {time_end - time_start:.8f} sec')
        return result
    return wrap