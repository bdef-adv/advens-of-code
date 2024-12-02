#!/usr/bin/python
""" day 12 """
from pathlib import Path
import sys
from itertools import combinations

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def get_permutations(springs, damaged):
    """ get all valid combinations """
    count = springs.count('?')
    perms = set(combinations(".#"*count, count))
    results = set()

    for perm in perms:
        new_spring = ""
        new_damaged = []
        index = 0
        curr_damaged = 0

        for x, ch in enumerate(springs):
            if ch == '?':
                new_spring += perm[index]
                index += 1
            elif ch in '.#':
                new_spring += ch
            if new_spring[x] == '#':
                curr_damaged += 1
            elif new_spring[x] == '.':
                if curr_damaged:
                    new_damaged.append(curr_damaged)
                curr_damaged = 0

        if curr_damaged:
            new_damaged.append(curr_damaged)

        if tuple(new_damaged) == damaged:
            results.add(new_spring)

    return len(results)

def get_permutations_rec(springs, damaged, curr_damaged_size):
    if not springs:
        if not curr_damaged_size and not damaged:
            return 1
        if len(damaged) == 1 and curr_damaged_size == damaged[0]:
            return 1
        return 0

    spring = springs[0]
    springs = springs[1::]
    current_damaged = damaged[0] if damaged else 0
    next_damaged = damaged[1::] if len(damaged) > 1 else []

    if spring == '.':
        if curr_damaged_size == 0:
            return get_permutations_rec(springs, damaged, 0)
        if curr_damaged_size == current_damaged:
            return get_permutations_rec(springs, next_damaged, 0)
        return 0

    if spring == '#':
        if curr_damaged_size > current_damaged:
            return 0
        else:
            return get_permutations_rec(springs, damaged, curr_damaged_size + 1)

    if spring == '?':
        return get_permutations_rec('#'+springs, damaged, curr_damaged_size) + get_permutations_rec('.'+springs, damaged, curr_damaged_size)

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        spring_records = {}
        for _line in file:
            line = _line.rstrip()
            springs, damaged = line.split()
            spring_records[springs] = list(map(int, damaged.split(',')))

        result = 0
        for springs, damaged in spring_records.items():
            #print(springs, damaged)
            permutations = get_permutations_rec(springs, damaged, 0)
            print(permutations, springs, damaged)
            result += permutations

        return result


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()


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
