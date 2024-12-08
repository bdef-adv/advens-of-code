#!/usr/bin/python

from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def display_map(array):
    for y in array:
        print(y)

def get_reflections(puzzle):
    """ return largest reflection in puzzle based on mirror indexes """
    for i in range(1, len(puzzle)):
        # halve the fuckers
        top = puzzle[:i][::-1]
        bottom = puzzle[i:]

        count = 0
        for tr, br in list(zip(top, bottom)):
            if tr == br:
                count += 1
        if count in [len(top), len(bottom)]:
            return i

    return 0

def get_reflections_smudged(puzzle):
    """ return largest reflection in puzzle based on mirror indexes """
    for i in range(1, len(puzzle)):
        # halve the fuckers
        top = puzzle[:i][::-1]
        bottom = puzzle[i:]

        count = 0
        # compare each motherfucking line
        # top row #0 == bottom row #-1
        # top row #1 == bottom row #-2
        for tr, br in list(zip(top, bottom)):
            # compare top char #0 == bottom char #-1
            # if we see a difference we count it
            for tc, bc in list(zip(tr, br)):
                if tc != bc:
                    count += 1
        # only one difference, we found the stupid smudge and the reflection is the
        # only possible reflection
        if count == 1:
            return i

    return 0


def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        inputs = [l.splitlines() for l in file.read().split('\n\n')]

        results = 0
        for inp in inputs:
            inpverted = list(zip(*inp))

            horizontal = get_reflections(inp)
            vertical = get_reflections(inpverted)
            if horizontal > vertical:
                results += horizontal * 100
            else:
                results += vertical

        return results


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        inputs = [l.splitlines() for l in file.read().split('\n\n')]

        results = 0
        for puzzle in inputs:
            results += get_reflections_smudged(puzzle) * 100
            results += get_reflections_smudged(list(zip(*puzzle)))

        return results


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
