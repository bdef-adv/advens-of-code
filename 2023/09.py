#!/usr/bin/python

from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        sum_diffs = 0
        for _line in file:
            line = _line.rstrip()
            start_data = list(map(int, line.split()))
            differences = []
            all_differences = [start_data]
            data = start_data
            while True:
                for x, n in enumerate(data[1::]):
                    differences.append(n - data[x])
                data = differences
                all_differences.append(differences)
                if sum(differences) == 0:
                    break
                differences = []
            all_differences[-1].append(0)
            last = 0
            new_diffs = []
            for x, diff in enumerate(all_differences[-2::-1]):
                diff.append(diff[-1] + last)
                new_diffs.append(diff)
                last = diff[-1]
            sum_diffs += new_diffs[-1][-1]
        return sum_diffs


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        sum_diffs = 0
        for _line in file:
            line = _line.rstrip()
            start_data = list(map(int, line.split()))
            differences = []
            all_differences = [start_data]
            data = start_data
            while True:
                for x, n in enumerate(data[1::]):
                    differences.append(n - data[x])
                data = differences
                all_differences.append(differences)
                if sum(differences) == 0:
                    break
                differences = []
            all_differences[-1] = [0] + all_differences
            last = 0
            new_diffs = []
            print(all_differences)
            for x, diff in enumerate(all_differences[-2::-1]):
                print(x, diff)
                diff = [diff[0] - last] + diff
                new_diffs.append(diff)
                last = diff[0]
            sum_diffs += new_diffs[-1][0]
        return sum_diffs


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
