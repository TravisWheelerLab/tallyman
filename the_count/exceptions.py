class FASTAException(Exception):
    message: str

    def __init__(self, message: str):
        self.message = message

    def __str__(self):
        return f"{self.message}"


class MissingSequence(Exception):
    sequence: str

    def __init__(self, sequence: str):
        self.sequence = sequence

    def __str__(self):
        return f"{self.sequence}"
