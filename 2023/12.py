#!/usr/bin/python
""" day 12 """
from pathlib import Path
import sys
from itertools import combinations

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def is_valid(springs, damaged):
    """ Compare springs to damaged record to check if it is ok """
    damaged = list(map(int, damaged))
    curr_damaged = []

    curr = 0
    for ch in springs:
        if ch == '.':
            if curr:
                curr_damaged.append(curr)
            curr = 0
        elif ch == '?':
            return False
        elif ch == '#':
            curr += 1
    if curr:
        curr_damaged.append(curr)

    return curr_damaged == damaged

def get_permutations(springs, damaged):
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
        if new_damaged == list(map(int, damaged)):
            print(perm, new_spring, is_valid(new_spring, damaged), damaged, new_damaged)
            results.add(new_spring)
    print(springs, count, results)
    return perms

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        spring_records = {}
        for _line in file:
            line = _line.rstrip()
            springs, damaged = line.split()
            spring_records[springs] = damaged.split(',')

        result = 0
        for springs, damaged in spring_records.items():
            permutations = get_permutations(springs, damaged)
            result += len(permutations)
            #for perm in permutations:
            #    if is_valid(perm, damaged):
            #        result += 1

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
    #print(solution_part1(f"input.{FILENAME_TRUNC}.txt"))

    print("--- Part Two ---")
    print("Test result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.test.txt"))

    print("Result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
