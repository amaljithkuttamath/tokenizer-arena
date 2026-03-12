def fibonacci(n):
    """Return the nth Fibonacci number using iteration."""
    if n <= 0:
        return 0
    elif n == 1:
        return 1

    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


def main():
    for i in range(10):
        print(f"fib({i}) = {fibonacci(i)}")


if __name__ == "__main__":
    main()
