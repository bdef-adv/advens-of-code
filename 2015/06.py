#!/usr/bin/python

from pathlib import Path
import sys
import re

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        grid = [[0 for x in range(1000)] for y in range(1000)]
        for _line in file:
            line = _line.rstrip()
            re_pat = re.compile(r"([ \w]*) (\d+,\d+) ([ \w]*) (\d+,\d+)")
            instruction, position_init, _, position_end =  tuple(re_pat.findall(line))[0]
            y_init, x_init = map(int, position_init.split(','))
            y_end, x_end = map(int, position_end.split(','))
            #print(f"From (Y={y_init},{x_init}) to (X={y_end},{x_end}) => {instruction}")
            for y in range(y_init, y_end+1):
                for x in range(x_init, x_end+1):
                    match instruction:
                        case 'turn on':
                            grid[y][x] = 1
                        case 'turn off':
                            grid[y][x] = 0
                        case 'toggle':
                            grid[y][x] ^= 1

        return sum([sum(i) for i in grid])



def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        grid = [[0 for x in range(1000)] for y in range(1000)]
        for _line in file:
            line = _line.rstrip()
            re_pat = re.compile(r"([ \w]*) (\d+,\d+) ([ \w]*) (\d+,\d+)")
            instruction, position_init, _, position_end =  tuple(re_pat.findall(line))[0]
            y_init, x_init = map(int, position_init.split(','))
            y_end, x_end = map(int, position_end.split(','))
            #print(f"From (Y={y_init},{x_init}) to (X={y_end},{x_end}) => {instruction}")
            for y in range(y_init, y_end+1):
                for x in range(x_init, x_end+1):
                    match instruction:
                        case 'turn on':
                            grid[y][x] += 1
                        case 'turn off':
                            grid[y][x] = max(0, grid[y][x] - 1)
                        case 'toggle':
                            grid[y][x] += 2

        return sum([sum(i) for i in grid])


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
