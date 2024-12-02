#!/usr/bin/python
""" day 12 """
from pathlib import Path
import sys
from functools import cache

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""


# ce petit batard me fait gagner plusieurs minutes de calcul
@cache
def get_permutations_rec(springs, damaged, curr_damaged_size):
    """ we move character by character """
    # Search over, where are we?
    if not springs:
        if not curr_damaged_size and not damaged:
            return 1
        if len(damaged) == 1 and curr_damaged_size == damaged[0]:
            return 1
        return 0

    spring, springs = springs[0], springs[1::]
    current_damaged = damaged[0] if damaged else 0
    next_damaged = damaged[1::] if len(damaged) > 1 else []
    next_damaged = tuple(next_damaged)

    if spring == '.':
        if curr_damaged_size == 0:
            return get_permutations_rec(springs, damaged, 0)
        if curr_damaged_size == current_damaged:
            return get_permutations_rec(springs, next_damaged, 0)
        return 0

    if spring == '#':
        if curr_damaged_size > current_damaged:
            return 0

        return get_permutations_rec(springs, damaged, curr_damaged_size + 1)

    if spring == '?':
        return get_permutations_rec('#'+springs, damaged, curr_damaged_size) + get_permutations_rec('.'+springs, damaged, curr_damaged_size)


def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        result = 0
        for _line in file:
            line = _line.rstrip()
            springs, damaged = line.split()
            damaged = tuple(map(int, damaged.split(',')))
            result += get_permutations_rec(springs, damaged, 0)

        return result


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        result = 0
        for _line in file:
            line = _line.rstrip()
            springs, damaged = line.split()
            springs = '?'.join([springs] * 5)
            damaged = tuple(map(int, damaged.split(','))) * 5
            result += get_permutations_rec(springs, damaged, 0)

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
