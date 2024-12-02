#!/usr/bin/python

from pathlib import Path
import sys

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

DIRECTION_TO_STR = {
    Point(0, 1): "right",
    Point(0, -1): "left",
    Point(1, 0): "down",
    Point(-1, 0): "up"
}

STR_TO_DIRECTION = {
    "right": Point(0, 1),
    "left": Point(0, -1),
    "down": Point(1, 0),
    "up": Point(-1, 0)
}

def display_map(puzzle, beams=None):
    """ display 2d map """
    if not beams:
        for l in puzzle:
            print(l)
        return None
    nbeams = set([l if isinstance(l, Point) else l[0] for l in beams])
    count = 0
    for y, l in enumerate(puzzle):
        for x, ch in enumerate(l):
            if Point(y, x) in nbeams:
                print('#', end="")
                count += 1
            else:
                print(ch, end="")
        print()
    return count

def count_beams(beams):
    """ How much beams are lit up """
    return len(set([l if isinstance(l, Point) else l[0] for l in beams]))

def get_path(puzzle, position, direction, already_lit_up):
    """ recursive function to advance through the puzzle """
    next_pos = position + direction
    #print(f"Current position: {position}, Direction: {direction}, Next position: {next_pos}")
    if (next_pos.y < 0 or next_pos.x < 0 or
        next_pos.y >= len(puzzle) or next_pos.x >= len(puzzle[0]) or
        (next_pos, direction) in already_lit_up):
        #print(f"END OF PUZZLE: {already_lit_up}")
        return already_lit_up

    next_char = puzzle[next_pos.y][next_pos.x]

    already_lit_up.add((next_pos, direction))
    if next_char == '.':
        return get_path(puzzle, next_pos, direction, already_lit_up)

    if next_char == '\\':
        next_direction = direction
        match DIRECTION_TO_STR[direction]:
            case "right":
                next_direction = STR_TO_DIRECTION["down"]
            case "left":
                next_direction = STR_TO_DIRECTION["up"]
            case "down":
                next_direction = STR_TO_DIRECTION["right"]
            case "up":
                next_direction = STR_TO_DIRECTION["left"]
        return get_path(puzzle, next_pos, next_direction, already_lit_up)

    if next_char == '/':
        next_direction = direction
        match DIRECTION_TO_STR[direction]:
            case "right":
                next_direction = STR_TO_DIRECTION["up"]
            case "left":
                next_direction = STR_TO_DIRECTION["down"]
            case "down":
                next_direction = STR_TO_DIRECTION["left"]
            case "up":
                next_direction = STR_TO_DIRECTION["right"]
        return get_path(puzzle, next_pos, next_direction, already_lit_up)

    if next_char == '-':
        if DIRECTION_TO_STR[direction] in ["down", "up"]:
            return get_path(puzzle, next_pos, STR_TO_DIRECTION["left"], already_lit_up) | get_path(puzzle, next_pos, STR_TO_DIRECTION["right"], already_lit_up)
        return get_path(puzzle, next_pos, direction, already_lit_up)

    if next_char == '|':
        if DIRECTION_TO_STR[direction] in ["left", "right"]:
            return get_path(puzzle, next_pos, STR_TO_DIRECTION["up"], already_lit_up) | get_path(puzzle, next_pos, STR_TO_DIRECTION["down"], already_lit_up)
        return get_path(puzzle, next_pos, direction, already_lit_up)



def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        puzzle = file.read().splitlines()
        beams = get_path(puzzle, Point(0, -1), STR_TO_DIRECTION["right"], set())
        return count_beams(beams)



def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        puzzle = file.read().splitlines()
        len_y = len(puzzle)
        len_x = len(puzzle[0])
        beams = set()
        for y, line in enumerate(puzzle):
            for x, _ in enumerate(line):
                # Left edge - go right
                if x == 0:
                    beams.add(count_beams(get_path(puzzle, Point(y, x-1), STR_TO_DIRECTION["right"], set())))
                # Right edge - go left
                if x == len_x - 1:
                    beams.add(count_beams(get_path(puzzle, Point(y, x+1), STR_TO_DIRECTION["left"], set())))
                # Top row - go down
                if y == 0:
                    beams.add(count_beams(get_path(puzzle, Point(y-1, x), STR_TO_DIRECTION["down"], set())))
                # Bottom row - go up
                if y == len_y - 1:
                    beams.add(count_beams(get_path(puzzle, Point(y+1, x), STR_TO_DIRECTION["up"], set())))
        return max(beams)


if __name__ == "__main__":
    print("--- Part One ---")
    print("Test result:")
    print(solution_part1(f"input.{FILENAME_TRUNC}.test.txt"))

    print("Result:")
    print(solution_part1(f"input.{FILENAME_TRUNC}.txt"))

    print("--- Part Two ---")
    print("Test result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.test.txt"))

    print("Result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
