#!/usr/bin/python

from pathlib import Path
import sys
from collections import Counter

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def display_map(puzzle):
    for line in puzzle:
        print(''.join(line))

def get_line(line):
    new_line = line
    moved = True
    while moved:
        moved = False
        count_gap = 0
        curr_line = []
        for x, ch in enumerate(new_line):
            if ch == '.':
                count_gap += 1
            elif ch == 'O' and count_gap:
                curr_line += ['O'] + ['.']*count_gap
                count_gap = 0
                curr_line += new_line[x+1::]
                moved = True
                break
            elif count_gap:
                curr_line += ['.'] * (count_gap) + [ch]
                count_gap = 0
            else:
                curr_line += [ch]
        if count_gap:
            curr_line += ['.'] * count_gap
        new_line = curr_line
    return new_line

def get_slipped_map_north(puzzle):
    flipped = list(zip(*puzzle))
    new_map = []
    len_line = len(flipped[0])
    for line in flipped:
        new_map.append(get_line(line))
    return list(zip(*new_map))

def get_slipped_map_south(puzzle):
    flipped = list(zip(*puzzle[::-1]))
    new_map = []
    len_line = len(flipped[0])
    for line in flipped:
        new_map.append(get_line(line))
    return list(zip(*new_map))[::-1]

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        puzzle = []
        for _line in file:
            line = _line.rstrip()
            puzzle.append(line)

        puzzle = get_slipped_map_north(puzzle)
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

        len_puzzle = len(puzzle)
        for i in range(1000):
            puzzle = get_slipped_map_north(puzzle) # Flipped north
            #print("North")
            #display_map(puzzle)

            #print("West")
            new_puzzle = []
            for line in puzzle:
                new_puzzle.append(get_line(line))
            puzzle = new_puzzle

            #display_map(puzzle)

            #print("South")
            puzzle = get_slipped_map_south(puzzle)
            #display_map(puzzle)

            #print("East")
            puzzle = [y[::-1] for y in puzzle]
            new_puzzle = []
            for line in puzzle:
                new_puzzle.append(get_line(line))
            puzzle = [y[::-1] for y in new_puzzle]
            #display_map(puzzle)


            count = 0

            for x, line in enumerate(puzzle):
                count += Counter(line)['O'] * (len_puzzle - x)

            print(f"\rCount: {count} Iteration: {i}", end="")

        print()

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
