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
        inputs = []
        curr_input = []
        for _line in file:
            line = _line.rstrip()
            if line:
                curr_input.append(line)
            else:
                inputs.append(curr_input)
                curr_input = []
        if curr_input:
            inputs.append(curr_input)
        
        results = 0
        nb_inputs = len(inputs)
        for index in range(0, nb_inputs):
            left = inputs[index]
            vertical_map = ["" for i in range(len(left[0]))]
            
            horizontal_found = None
            last_horizontal = left[0]
            for y, line in enumerate(left):
                if line == last_horizontal and y != 0:
                    horizontal_found = y
                last_horizontal = line
                for x, ch in enumerate(line):
                    vertical_map[x] += ch
            
            vertical_found = None
            last_vertical = vertical_map[0]
            for y, line in enumerate(vertical_map):
                if line == last_vertical and y != 0:
                    vertical_found = y
                    break
                last_vertical = line

            print(left)
            print(vertical_map)
            if horizontal_found < vertical_found:
                count = 0
                for x, line in enumerate(left[horizontal_found+1::]):
                    if line == left[horizontal_found - 1 - x]:
                        count += 1
                    else:
                        break
                print(f"horizontal_left_count = {count}")
                results += count 
            elif vertical_found < horizontal_found:
                count = 0
                for x, line in enumerate(vertical_map[vertical_found+1::]):
                    if line == vertical_map[vertical_found - 1 - x]:
                        count += 1
                    else:
                        break
                print(f"vertical_top_count = {count}")
                results += count * 100

            print(f"Horizontal: {horizontal_found}; Vertical: {vertical_found}")

        return results


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
