import time

from rusty_primes.rusty_primes import get_primes

from pythonic_primes import get_primes_pythonic
from utils import time_function

_NUMBER_OF_PRIMES_TO_GET = 1_000_000
_SHOULD_PRINT = False

@time_function
def python_implementation():
    get_primes_pythonic(_NUMBER_OF_PRIMES_TO_GET, _SHOULD_PRINT)


@time_function
def rust_implementation():
    get_primes(_NUMBER_OF_PRIMES_TO_GET, _SHOULD_PRINT)


def main():
    """
    | 1 000 000 primes, no printing |
    Function: 'test_pythonic' took: 1.58432961 sec
    Function: 'test_rustic' took: 0.16995263 sec
    """
    python_implementation()
    time.sleep(2)
    rust_implementation()

if __name__ == '__main__':
    main()