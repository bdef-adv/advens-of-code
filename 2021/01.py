#!/usr/bin/python

from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def solution_part1(filename):
    with open(filename, "r") as file:
        increased = 0
        previous = None

        for _line in file:
            line = _line.rstrip()
            lineint = int(line)
            if previous is None:
                previous = lineint
                continue

            if lineint > previous:
                increased += 1

            previous = lineint

        return increased

def solution_part2(filename):
    with open(filename, "r") as file:
        values = []
        for i, _line in enumerate(file):
            line = _line.rstrip()
            lineint = int(line)
            values.append(lineint)

        data = []
        len_values = len(values)

        findex = 0
        while True:
            if findex + 3 > len_values:
                break
            
            #print(findex, field, findex + 3, len_values)
            
            # Sum all the value in the sliding window and add it to the dataset
            cursum = sum(values[findex:findex + 3:])
            data.append(cursum)
            findex += 1

        # same as part one
        increased = 0
        previous = None

        for value in data:
            if previous is None:
                previous = value
                continue

            if value > previous:
                increased += 1

            previous = value

        return increased


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
