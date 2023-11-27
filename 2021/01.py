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

        fields = ''.join([chr(x) for x in range(ord('A'), ord('Z')+1)])
        data = {a: 0 for a in fields}
        
        len_fields = len(fields)
        len_values = len(values)

        current_findex = 0
        current_field = fields[current_findex]
        current_index = 0

        for findex, field in enumerate(fields):
            if findex >= len_values:
                break
            print(findex, field)
            for val in values[findex:findex + 3:]:
                print(f"Adding {val} to {field}")
                data[field] += val
            current_index = current_index + 1

        print(data)
        increased = 0
        previous = None

        # same algorithm as part one
        for field, value in data.items():
            if previous is None:
                previous = value
                continue

            if value > previous:
                increased += 1

            previous = value

        return increased

def solution_part2_test(filename):
    with open(filename, "r") as file:
        fields = ''.join([chr(x) for x in range(ord('A'), ord('Z')+1)])
        data = {a: 0 for a in fields}

        len_fields = len(fields)

        field_index = 0
        current_index = 0

        for i, _line in enumerate(file):
            line = _line.rstrip()
            lineint = int(line)

            # create the list of letters on which to append
            letters = [
                fields[field_index % len_fields],
            ]
            # add letters depending on where we are
            for x in range(current_index):
                letters.append(fields[(field_index + 1 + x) % len_fields])

            print(i, current_index, field_index, lineint, letters)

            # prepare next iteration
            if current_index == 2:
                field_index = (field_index + 1) % len_fields
            current_index = (current_index + 1) % 3

            # update sums 
            for letter in letters:
                data[letter] += lineint

        increased = 0
        previous = None

        print(data)

        # same algorithm as part one
        for field, value in data.items():
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
    #print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
