import math


def is_prime_pythonic(n):
    if n <= 1:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    # Check divisibility from 3 to sqrt(n), skipping even numbers
    for i in range(3, int(math.sqrt(n)) + 1, 2):
        if n % i == 0:
            return False

    return True


def get_primes_pythonic(stop: int, should_print: bool) -> list[int]:
    primes = []
    current_number = 1

    while True:
        if is_prime_pythonic(current_number):
            primes.append(current_number)
            if should_print:
                print(f"Found prime: {current_number}")

        current_number += 1
        if current_number >= stop:
            break

    return primes