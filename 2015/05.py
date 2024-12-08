#!/usr/bin/python

from pathlib import Path
import sys
import re

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = "_2"

def is_nice(string):
    """ PART 1
    """
    for t in ["ab", "cd", "pq", "xy"]:
        if t in string:
            return False
    # count vowels
    vowels = 0
    couples = ''
    prev = ''
    for ch in string:
        if ch == prev:
            couples += ch
        if ch in "aeiou":
            vowels += 1
        prev = ch
        if vowels >= 3 and couples:
            return True

    return False

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        count = 0
        for _line in file:
            line = _line.rstrip()
            count += int(is_nice(line))
        return count


def is_nice_part2(string):
    """ PART 2
    """
    re_criteria1 = re.compile(r"((.)(.)).*\1")
    re_criteria2 = re.compile(r"((.).\2)")
    if re_criteria1.findall(string) and re_criteria2.findall(string):
        return True

    return False


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        count = 0
        for _line in file:
            line = _line.rstrip()
            count += int(is_nice_part2(line))
        return count

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
