#!/usr/bin/python

from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

MOVEMENTS = {'^': (-1, 0), '>': (0, 1), '<': (0, -1), 'v': (1, 0)}

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()
            y, x = 0, 0
            visited = {(y, x)}
            for ch in line:
                y, x = tuple(map(sum, zip((y, x), MOVEMENTS[ch])))
                visited.add((y, x))
            return len(visited)


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()
            santa = (0, 0)
            robo = (0, 0)
            visited = {santa}
            for index, ch in enumerate(line):
                if index % 2 == 0:
                    santa = tuple(map(sum, zip(santa, MOVEMENTS[ch])))
                    visited.add(santa)
                else:
                    robo = tuple(map(sum, zip(robo, MOVEMENTS[ch])))
                    visited.add(robo)

            return len(visited)


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
