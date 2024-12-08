#!/usr/bin/python

from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def hash(string):
    current_value = 0
    for ch in string:
        current_value += ord(ch)
        current_value *= 17
        current_value %= 256
    return current_value


def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()
            return sum((hash(y) for y in line.split(',')))


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        boxes = [{} for i in range(256)]
        for _line in file:
            line = _line.rstrip()
            for instr in line.split(','):
                if '=' in instr:
                    label, value = instr.split('=')
                    curr_box = hash(label)
                    boxes[curr_box][label] = value
                elif '-' in instr:
                    label, _ = instr.split('-')
                    curr_box = hash(label)
                    if label in boxes[curr_box]:
                        del boxes[curr_box][label]

            result = 0
            for x, box in enumerate(boxes):
                for y, lv in enumerate(box.items()):
                    l, v = lv
                    result += (1 + x) * (1+y) * int(v) 

            return result



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
