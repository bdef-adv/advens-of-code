#!/usr/bin/python

from pathlib import Path
import sys

INPUT_PATH = str(Path(__file__).parent.resolve()) + '/inputs'
FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

class Point:
    """ Defines a 2D point """
    def __init__(self, y, x=None):
        if x is None:
            self.y, self.x = y
        else:
            self.y = y
            self.x = x

    def __eq__(self, other):
        return (self.x, self.y) == (other.x, other.y)

    def __repr__(self):
        return f"{self.y},{self.x}"

    def __hash__(self):
        return hash(repr(self))

    def __add__(self, other):
        return Point(self.y + other.y, self.x + other.x)

    def __lt__(self, other):
        if self.y < other.y:
            return True
        if self.y == other.y:
            return self.x < other.x
        return False

    def __mul__(self, other):
        if isinstance(other, int):
            return Point(self.y * other, self.x * other)
        return Point(self.y * other.y, self.x * other.x)

DIRECTION_TO_STR = {
    Point(0, 1): "right",
    Point(0, -1): "left",
    Point(1, 0): "down",
    Point(-1, 0): "up"
}
STR_TO_DIRECTION = {r: l for l, r in DIRECTION_TO_STR.items()}

def go_in_direction(puzzle, position, direction, steps=0, positions=[], already_visited_twice=[]):
    #print(f"At {position} Going {DIRECTION_TO_STR[direction]} (steps={steps}) (positions={positions})")
    if steps == 6:
        return positions + [position]

    if position in already_visited_twice:
        return []


    if position in positions:
        already_visited_twice.append(position)

    next_pos = position + direction
    next_char = puzzle[next_pos.y][next_pos.y]
    if next_char == '#':
        return []

    possible_directions = []
    for direction in STR_TO_DIRECTION:
        nextpos = position + STR_TO_DIRECTION[direction]
        if puzzle[nextpos.y][nextpos.x] == '.':
            possible_directions.append(direction)

    for direction in possible_directions:
        positions += list(go_in_direction(puzzle, next_pos, STR_TO_DIRECTION[direction], steps + 1, positions, already_visited_twice))

    return positions

def display_map(puzzle):
    import os 
    os.system('clear')
    for y in puzzle:
        print(y)


def get_positions(puzzle, position, steps):
    positions = set()
    positions.add(position)

    for i in range(steps):
        new_positions = set()
        for position in positions:
            for direction in DIRECTION_TO_STR:
                new_point = position + direction
                if (new_point.y >= len(puzzle) or new_point.y < 0 or 
                    new_point.x >= len(puzzle[0]) or new_point.x < 0):
                    continue
                if puzzle[new_point.y][new_point.x] == '.':
                    new_positions.add(new_point)
        positions = new_positions
    return new_positions


def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        puzzle = file.read().splitlines()

        position = None
        for y, line in enumerate(puzzle):
            for x, ch in enumerate(line):
                if ch == 'S':
                    position = Point(y, x)
                    puzzle[y] = puzzle[y].replace('S', '.')
                    break

        positions = get_positions(puzzle, position, 64)
        return len(positions)



def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()


if __name__ == "__main__":
    print("--- Part One ---")
    print("Test result:")
    print(solution_part1(f"{INPUT_PATH}/input.{FILENAME_TRUNC}.test.txt"))

    print("Result:")
    print(solution_part1(f"{INPUT_PATH}/input.{FILENAME_TRUNC}.txt"))

    print("--- Part Two ---")
    print("Test result:")
    print(solution_part2(f"{INPUT_PATH}/input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.test.txt"))

    print("Result:")
    print(solution_part2(f"{INPUT_PATH}/input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
