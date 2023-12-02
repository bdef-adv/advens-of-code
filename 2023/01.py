#!/usr/bin/python

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
    with open(filename, "r") as file:
        result = 0
        for _line in file:
            line = list(filter(str.isdigit,_line.rstrip()))
            result += int(line[0] + line[-1])
        return result

def get_digits(string):
    places = {}
    for num, trad in TRADUCTION.items():
        if num in string:
            places[trad] = list(filter(lambda v: v != -1, [string.find(num), string.rfind(num)]))
            print(string.rstrip(), num, trad, places[trad])

    min_values = list(sorted(places, key=lambda v: min(places[v])))
    max_values = list(sorted(places, key=lambda v: max(places[v])))
    #print(min_values, max_values)
    return min_values[0] * 10 + max_values[-1]

def solution_part2(filename):
    with open(filename, "r") as file:
        result = 0
        for _line in file:
            result += get_digits(_line.rstrip())
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
