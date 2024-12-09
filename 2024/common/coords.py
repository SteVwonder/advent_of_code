class Coordinates:
    def __init__(self, row: int, col: int):
        self.row = row
        self.col = col

    def __add__(self, other: 'Coordinates') -> 'Coordinates':
        return Coordinates(self.row + other.row, self.col + other.col)

    def __sub__(self, other: 'Coordinates') -> 'Coordinates':
        return Coordinates(self.row - other.row, self.col - other.col)

    def __neg__(self) -> 'Coordinates':
        return Coordinates(-self.row, -self.col)

    def __eq__(self, other):
        return self.row == other.row and self.col == other.col

    def __hash__(self):
        return hash((self.row, self.col))

    def __repr__(self):
        return f'({self.row}, {self.col})'

