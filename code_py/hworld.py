"""Module providing basic print function."""


def say_hello(obj="World"):
    """Function that return a greeting message."""
    return "Hello, " + obj + "!"


if __name__ == "__main__":
    print(say_hello())
    print(say_hello("Python"))
