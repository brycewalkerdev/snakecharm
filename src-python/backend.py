import sys


def main(operation, args):
    """Main entry point for the backend.

    This function is called from the frontend to execute operations in the
    backend. It is responsible for setting up the environment, executing the
    function, and returning the result.

    Args:
        operation (str): The name of the function to execute in the backend.
        *args: Positional arguments to pass to the function.

    Returns:
        The result of the function call.

    """
    match operation:
        case "version":
            print(sys.version)
        case "addition":
            addition(*args)
        case _:
            raise Exception(f"Unknown operation: {operation}")


def addition(*args):
    """Addition function.

    Args:
        *args: Positional arguments to add.

    Returns:
        The sum of the arguments.

    """
    try:
        print(sum(map(int, args)))
    except:
        raise ValueError("Arguments must be numbers.")


main(sys.argv[1], sys.argv[2:])

input("Press the any key: ")
