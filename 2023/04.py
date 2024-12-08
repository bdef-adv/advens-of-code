#!/usr/bin/python

from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = "_2"

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        final_score = 0
        for _line in file:
            line = _line.rstrip()
            card, data = line.split(": ")
            left, right = map(str.split, data.split(' | '))
            points = 0
            for number in right:
                if number in left:
                    if not points:
                        points = 1
                    else:
                        points *= 2
            final_score += points
            #print(f"{line}: {left}, {right} ({points}) {numbers}")

        return final_score


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        final_score = 0
        for _line in file:
            line = _line.rstrip()
            card, data = line.split(": ")
            left, right = map(str.split, data.split(' | '))
            points = 0
            numbers = []
            for number in right:
                if number in left:
                    if not points:
                        points = 1
                    else:
                        points *= 2
                    numbers.append(number)
            final_score += points

        return final_score


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
