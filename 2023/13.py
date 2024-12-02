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
    reflections = []
    indexes = []
    len_puzzle = len(puzzle)

    # find mirror positions
    last_line = puzzle[0]
    for y, line in enumerate(puzzle[1::]):
        if line == last_line:
            indexes.append(y + 1)
        last_line = line

    for index in indexes:
        count = 0
        valid = False
        i = index - 1
        for y, line in enumerate(puzzle[index - 1::-1]):
            if index + y >= len_puzzle:
                count += 1
                valid = True
            elif line == puzzle[index + y]:
                count += 1
                if i == 0:
                    valid = True
            else:
                break
            i -= 1
        if valid:
            reflections.append(count)

    if not reflections:
        return 0
    return min(reflections)

def get_reflections_bruteforce(puzzle):
    """ return largest reflection in puzzle based on mirror indexes """
    vertical_map = list(zip(*puzzle))

    base_reflection = get_reflections(puzzle)
    base_reflection_vert = get_reflections(vertical_map)

    for y, line in enumerate(puzzle):
        for x, ch in enumerate(line):
            new_puzzle_horiz = [[x for x in y] for y in puzzle]
            new_puzzle_vert = [[x for x in y] for y in vertical_map]
            if ch == '.':
                new_puzzle_horiz[y][x] = '#'
            elif ch == '#':
                new_puzzle_horiz[y][x] = '.'

            if vertical_map[x][y] == '.':
                new_puzzle_vert[x][y] = '#'
            elif vertical_map[x][y] == '#':
                new_puzzle_vert[x][y] = '.'

            new_puzzle_horiz = [''.join(_line) for _line in new_puzzle_horiz]
            new_puzzle_vert = [''.join(_line) for _line in new_puzzle_vert]
            
            new_reflection_horiz = get_reflections(new_puzzle_horiz)
            new_reflection_vert = get_reflections(new_puzzle_vert)

            #print(f"New reflections: {new_reflection_horiz},{new_reflection_vert} (base={base_reflection},{base_reflection_vert}) (Line={new_puzzle_horiz[y]};vert={new_puzzle_vert[x]})")

            if new_reflection_horiz and new_reflection_horiz != base_reflection:
                #print(f"Horizontal new_reflection found: {new_reflection_horiz} vs {base_reflection}")
                return new_reflection_horiz * 100
            elif new_reflection_vert and new_reflection_vert != base_reflection_vert:
                #print(f"Vertical new_reflection found: {new_reflection_vert} vs {base_reflection_vert}")
                return new_reflection_vert

    if base_reflection > base_reflection_vert:
        return base_reflection * 100
    else:
        return base_reflection_vert


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

            horizontal_found = []
            last_horizontal = left[0]
            for y, line in enumerate(left):
                if line == last_horizontal and y != 0:
                    horizontal_found.append(y)
                last_horizontal = line
                for x, ch in enumerate(line):
                    vertical_map[x] += ch

            horizontal = get_reflections(left)
            vertical = get_reflections(vertical_map)
            if horizontal > vertical:
                results += horizontal * 100
            else:
                results += vertical

        return results



def solution_part2(filename):
    """ PART 2
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
        count = 0
        nb_inputs = len(inputs)
        for index in range(0, nb_inputs):
            left = inputs[index]
            vertical_map = ["" for i in range(len(left[0]))]

            horizontal_found = []
            last_horizontal = left[0]
            for y, line in enumerate(left):
                if line == last_horizontal and y != 0:
                    horizontal_found.append(y)
                last_horizontal = line
                for x, ch in enumerate(line):
                    vertical_map[x] += ch

            #print("---")
            #print("Horizontal:")
            results += get_reflections_bruteforce(left)

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
