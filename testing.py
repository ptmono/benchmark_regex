import timeit
import functools

TIMEITS = {}


def execute_tests():
    for name, (times, func, args) in TIMEITS.items():
        _func = functools.partial(func, *args)
        t = timeit.Timer(_func)
        dd = t.timeit(times)
        print(name + ": " + dd)


def ttimeit(times, *args):
    def execute_ttimeit(func):
        TIMEITS[func.__name__] = (times, func, args)

        def wrapper(*args):
            func(*args)
            return
        return wrapper
    return execute_ttimeit
