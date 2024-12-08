#!/usr/bin/python
""" day 1 """
from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = "_2"

TRADUCTION = {
    "zero": 0, "one": 1, "two": 2, "three": 3, "four": 4,
    "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9,
    "0": 0, "1": 1, "2": 2, "3": 3, "4": 4, "5": 5, "6": 6,
    "7": 7, "8": 8, "9": 9
}

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        result = 0
        for _line in file:
            line = list(filter(str.isdigit,_line.rstrip()))
            result += int(line[0] + line[-1])
        return result

def get_digits(string):
    """ part 2 answer
    """
    places = {}
    for num, trad in TRADUCTION.items():
        if num in string:
            if trad not in places:
                places[trad] = []
            places[trad].append(string.find(num))
            places[trad].append(string.rfind(num))
    min_value = list(sorted(places, key=lambda v: min(places[v])))[0]
    max_value = list(sorted(places, key=lambda v: max(places[v])))[-1]
    return min_value * 10 + max_value

def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        result = 0
        for _line in file:
            result += get_digits(_line)
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
