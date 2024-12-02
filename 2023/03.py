#!/usr/bin/python

from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def solution_part1(filename):
    """ PART 1
    """
    twod_map = []
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()
            twod_map.append(line + '.')

    parts_number = []

    size_y = len(twod_map)
    size_x = len(twod_map[0])
    for y in range(size_y):
        start_x = 0
        end_x = 0
        current_number = ""
        for x in range(size_x):
            if twod_map[y][x].isdigit():
                if not current_number:
                    start_x = x
                current_number += twod_map[y][x]
            elif twod_map[y][x] not in ['.'] and current_number:
                end_x = x - 1
                parts_number.append(int(current_number))
                current_number = ""
            elif twod_map[y][x] == '.':
                end_x = x - 1
                found = False
                for _y in range(y-1 if y-1 >= 0 else 0, y+2 if y+2 <= size_y else size_y):
                    if not current_number: break

                    for _x in range(start_x-1 if start_x -1 >= 0 else 0, end_x+2 if end_x+2<= size_x else size_x):
                        if current_number and twod_map[_y][_x] not in ['.'] and not twod_map[_y][_x].isdigit():
                            parts_number.append(int(current_number))
                            found = True
                            break
                    if found:
                        break
                current_number = ""
    
    return sum(parts_number)


def solution_part2(filename):
    """ PART 2
    """
    twod_map = []
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()
            twod_map.append(line + '.')

    parts_number = {}

    size_y = len(twod_map)
    size_x = len(twod_map[0])
    for y in range(size_y):
        start_x = 0
        end_x = 0
        current_number = ""
        for x in range(size_x):
            if twod_map[y][x].isdigit():
                if not current_number:
                    start_x = x
                current_number += twod_map[y][x]
            elif twod_map[y][x] in ['*'] and current_number:
                end_x = x - 1
                if (y, x) not in parts_number: parts_number[(y,x)] = []
                parts_number[(y,x)].append(int(current_number))
                current_number = ""
            elif twod_map[y][x] == '.':
                end_x = x - 1
                found = False
                for _y in range(y-1 if y-1 >= 0 else 0, y+2 if y+2 <= size_y else size_y):
                    if not current_number: break

                    for _x in range(start_x-1 if start_x -1 >= 0 else 0, end_x+2 if end_x+2<= size_x else size_x):
                        if current_number and twod_map[_y][_x] in ['*']:
                            if (_y, _x) not in parts_number: parts_number[(_y,_x)] = []
                            parts_number[(_y, _x)].append(int(current_number))
                            found = True
                            break
                    if found:
                        break
                current_number = ""
    
    final_count = 0
    for _, gears in parts_number.items():
        if len(gears) == 2:
            final_count += gears[0]*gears[1]
    return final_count


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
