#!/usr/bin/python

from pathlib import Path
import sys
from itertools import combinations

INPUT_PATH = str(Path(__file__).parent.resolve()) + '/inputs'
FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""


def solution_part1(filename):
    """ PART 1
    """
    offset = 1
    with open(filename, "r", encoding="utf-8") as file:
        universe = []
        vertical_stars_indexes = []
        stars = []
        for y, _line in enumerate(file):
            line = _line.rstrip()
            universe.append(line)
            if not vertical_stars_indexes:
                vertical_stars_indexes = [0 for ch in line]
            for x, ch in enumerate(line):
                if ch == '#':
                    vertical_stars_indexes[x] += 1

        # find stars
        offset_y = 0
        for y, line in enumerate(universe):
            offset_x = 0
            if '#' not in line:
                offset_y += offset
            for x, ch in enumerate(line):
                if vertical_stars_indexes[x] == 0:
                    offset_x += offset
                    continue
                if ch == '#':
                    stars.append((y + offset_y, x + offset_x))

        # calculate differences
        differences = []
        for iterat in combinations(stars, 2):
            starleft, starright = iterat
            y_left, x_left = starleft
            y_right, x_right = starright

            differences.append(abs(x_right - x_left) + abs(y_right - y_left))

        return sum(differences)

def solution_part2(filename):
    """ PART 2
    """
    offset = 1000000
    with open(filename, "r", encoding="utf-8") as file:
        universe = []
        vertical_stars_indexes = []
        stars = []
        for y, _line in enumerate(file):
            line = _line.rstrip()
            universe.append(line)
            if not vertical_stars_indexes:
                vertical_stars_indexes = [0 for ch in line]
            # find empty vertical universe spots by counting where the stars are
            for x, ch in enumerate(line):
                if ch == '#':
                    vertical_stars_indexes[x] += 1

        # find stars
        offset_y = 0
        for y, line in enumerate(universe):
            offset_x = 0
            if '#' not in line:
                offset_y += offset - 1
            for x, ch in enumerate(line):
                if vertical_stars_indexes[x] == 0:
                    offset_x += offset - 1
                    continue
                if ch == '#':
                    stars.append((y + offset_y, x + offset_x))

        # calculate differences
        differences = []
        for iterat in combinations(stars, 2):
            starleft, starright = iterat
            y_left, x_left = starleft
            y_right, x_right = starright

            differences.append(abs(x_right - x_left) + abs(y_right - y_left))

        return sum(differences)


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
