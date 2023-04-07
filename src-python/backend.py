import sys
import os
import string


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
        case "ingest":
            print(ingest_article())
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
        print(sum(map(float, args)))
    except:
        print("Invalid input. Arguments must be numbers.")


def resource_path(relative_path):
    """Get absolute path to resource, works for dev and for PyInstaller"""
    try:
        # PyInstaller creates a temp folder and stores path in _MEIPASS
        base_path = sys._MEIPASS
    except Exception:
        base_path = os.path.abspath(".")

    return os.path.join(base_path, relative_path)


def ingest_article():
    """Ingest an article.

    Returns:
        Sanitize article string

    """

    with open(resource_path("resources/pp.txt"), "r") as f:
        raw = f.read()
        text = sanitize(raw)
        return text


# Clean File and return new string
def sanitize(f):  # (file) => String
    # Replace hyphens with spaces
    f = f.replace("-", " ")
    # Remove punctuation
    f = f.translate(str.maketrans("", "", string.punctuation))
    # Make lowercase
    f = f.lower()

    return f


main(sys.argv[1], sys.argv[2:])

# input("Press the any key: ")
