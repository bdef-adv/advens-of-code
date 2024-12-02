#!/usr/bin/python

from pathlib import Path
import sys

INPUT_PATH = str(Path(__file__).parent.resolve()) + '/inputs'
FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

sys.setrecursionlimit(10000)

class Point:
    """ Defines a 2D point """
    def __init__(self, y, x):
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

DIRECTION_TO_STR = {
    Point(0, 1): "right",
    Point(0, -1): "left",
    Point(1, 0): "down",
    Point(-1, 0): "up"
}
STR_TO_DIRECTION = {r: l for l, r in DIRECTION_TO_STR.items()}

CURR_MIN = None

def get_path(puzzle, direction, position=Point(0,0), count_direction=0, curr_heat_loss=0, already_visited=set()):
    """ calculate path length from start point"""
    global CURR_MIN
    next_pos = position + direction
    len_puzzle = len(puzzle)
    len_puzzle_x = len(puzzle[0])
    if next_pos.y == len_puzzle - 1 and next_pos.x == len_puzzle_x - 1:
        print(f"At position {position} (Moving {DIRECTION_TO_STR[direction]}) to {next_pos}. {CURR_MIN} ({curr_heat_loss}) ({sorted(already_visited)})")
        CURR_MIN = min(CURR_MIN, curr_heat_loss) if CURR_MIN else curr_heat_loss
        return CURR_MIN
    #if CURR_MIN and curr_heat_loss > CURR_MIN:
    #    return 99999999
    if next_pos.y < 0 or next_pos.x < 0:
        return 999999999
    if next_pos.x >= len_puzzle_x:
        return 999999999
    if next_pos.y >= len_puzzle:
        return 99999999
    if next_pos in already_visited:
        return 99999999

    next_heat_loss = puzzle[next_pos.y][next_pos.x]
    curr_heat_loss += int(next_heat_loss)
    already_visited.add(position)

    if DIRECTION_TO_STR[direction] in ["right", "left"]:
        if count_direction == 3:
            # Count is 3, go up or down
            return min(get_path(puzzle, STR_TO_DIRECTION["up"], next_pos, 0, curr_heat_loss, already_visited),
                       get_path(puzzle, STR_TO_DIRECTION["down"], next_pos, 0, curr_heat_loss, already_visited))
        return min(get_path(puzzle, STR_TO_DIRECTION["up"], next_pos, 0, curr_heat_loss, already_visited),
                   get_path(puzzle, STR_TO_DIRECTION["down"], next_pos, 0, curr_heat_loss, already_visited),
                   get_path(puzzle, direction, next_pos, count_direction + 1, curr_heat_loss, already_visited))
    if DIRECTION_TO_STR[direction] in ["down","up"]:
        if count_direction == 3:
            # Count is 3, go left or right
            return min(get_path(puzzle, STR_TO_DIRECTION["left"], next_pos, 0, curr_heat_loss, already_visited),
                       get_path(puzzle, STR_TO_DIRECTION["right"], next_pos, 0, curr_heat_loss, already_visited))
        return min(get_path(puzzle, STR_TO_DIRECTION["left"], next_pos, 0, curr_heat_loss, already_visited),
                   get_path(puzzle, STR_TO_DIRECTION["right"], next_pos, 0, curr_heat_loss, already_visited),
                   get_path(puzzle, direction, next_pos, count_direction + 1, curr_heat_loss, already_visited))

    return 999999998


def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        puzzle = file.read().splitlines()
        return min(get_path(puzzle, STR_TO_DIRECTION["right"]),
                   get_path(puzzle, STR_TO_DIRECTION["down"]))


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
    #print(solution_part1(f"{INPUT_PATH}/input.{FILENAME_TRUNC}.txt"))

    print("--- Part Two ---")
    print("Test result:")
    print(solution_part2(f"{INPUT_PATH}/input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.test.txt"))

    print("Result:")
    print(solution_part2(f"{INPUT_PATH}/input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
