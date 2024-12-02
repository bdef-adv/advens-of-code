#!/usr/bin/python

from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def solution_part1(filename, test=False):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            floor = 0
            line = _line.rstrip()
            for ch in line:
                if ch == '(': floor += 1
                else: floor -= 1
            return floor



def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()
            floor = 0
            for index, ch in enumerate(line):
                if ch == '(': floor += 1
                else: floor -= 1
                if floor == -1:
                    return index+1



if __name__ == "__main__":
    print("--- Part One ---")
    print("Test result:")
    print(solution_part1(f"input.{FILENAME_TRUNC}.test.txt", True))

    print("Result:")
    print(solution_part1(f"input.{FILENAME_TRUNC}.txt"))

    print("--- Part Two ---")
    print("Test result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.test.txt"))

    print("Result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
