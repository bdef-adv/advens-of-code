#!/usr/bin/python

from pathlib import Path
import sys
from collections import Counter

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def display_map(puzzle):
    for line in puzzle:
        print(line)

def get_slipped_map_north(puzzle):
    flipped = list(zip(*puzzle))
    new_map = []
    len_line = len(flipped[0])
    for line in flipped:
        new_line = line
        moved = True
        while moved:
            moved = False
            count_gap = 0
            curr_line = ""
            for x, ch in enumerate(new_line):
                if ch == '.':
                    count_gap += 1
                elif ch == 'O' and count_gap:
                    curr_line += 'O' + '.'*count_gap
                    count_gap = 0
                    curr_line += ''.join(new_line[x+1::])
                    moved = True
                    break
                elif count_gap:
                    curr_line += '.' * (count_gap) + ch
                    count_gap = 0
                else:
                    curr_line += ch
            if count_gap:
                curr_line += '.' * count_gap
            new_line = curr_line
        new_map.append(new_line)
    return new_map

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        puzzle = []
        for _line in file:
            line = _line.rstrip()
            puzzle.append(line)

        puzzle = get_slipped_map_north(puzzle)
        puzzle = [''.join(y) for y in list(zip(*puzzle))]
        count = 0
        len_puzzle = len(puzzle)

        for x, line in enumerate(puzzle):
            count += Counter(line)['O'] * (len_puzzle - x)
        return count


def solution_part2(filename):
    """ PART 2
    """
    from progressbar import progressbar
    with open(filename, "r", encoding="utf-8") as file:
        puzzle = []
        for _line in file:
            line = _line.rstrip()
            puzzle.append(line)

        for i in progressbar(range(4)):
            puzzle = get_slipped_map_north(puzzle)
            display_map([''.join(y) for y in list(zip(*puzzle))])
            print("---")

        count = 0
        len_puzzle = len(puzzle)

        for x, line in enumerate(puzzle):
            count += Counter(line)['O'] * (len_puzzle - x)
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
    #print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
