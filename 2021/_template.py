#!/usr/bin/python

from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem


def solution_part1(filename):
    with open(filename, "r") as file:
        for _line in file:
            line = _line.rstrip()


def solution_part2(filename):
    with open(filename, "r") as file:
        for _line in file:
            line = _line.rstrip()


if __name__ == "__main__":
    print("--- Part One ---")
    print("Test result:")
    print(solution_part1(f"input.{FILENAME_TRUNC}.test.txt"))

    print("Result:")
    print(solution_part1(f"input.{FILENAME_TRUNC}.txt"))

    print("--- Part Two ---")
    print("Test result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}_2.test.txt"))

    print("Result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}_2.txt"))

