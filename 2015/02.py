#!/usr/bin/python

from pathlib import Path
import sys

INPUT_PATH = str(Path(__file__).parent.resolve()) + '/inputs'
FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def solution_part1(filename):
    """ PART 1
    """
    result = 0
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()
            l, w, h = map(int, line.split('x'))
            result += 2*l*w + 2*w*h + 2*h*l + min(l*w, w*h, h*l)
    return result



def solution_part2(filename):
    """ PART 2
    """
    ribbon = 0
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()
            l, w, h = map(int, line.split('x'))
            ribbon += l*w*h + 2*sum(sorted([l, w, h])[0:2:])
    return ribbon

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
