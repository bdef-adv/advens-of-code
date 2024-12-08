#!/usr/bin/python

from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def display_map(array):
    for y in array:
        print(y)

def _solution_part1(filename):
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

            vertical_found = []
            last_vertical = vertical_map[0]
            for y, line in enumerate(vertical_map):
                if line == last_vertical and y != 0:
                    vertical_found.append(y)
                last_vertical = line

            horiz_count = [0 for i in horizontal_found]
            for curr, _horizontal_found in enumerate(horizontal_found):
                for x, line in enumerate(left[_horizontal_found - 2::-1]):
                    if _horizontal_found + 1 + x >= len(left):
                        horiz_count[curr] += 1
                        break
                    if line == left[_horizontal_found + 1 + x]:
                        horiz_count[curr] += 1

            vert_count = [0 for i in vertical_found]
            for curr, _vertical_found in enumerate(vertical_found):
                for x, line in enumerate(vertical_map[_vertical_found - 2::-1]):
                    if _vertical_found + 1 + x >= len(vertical_map):
                        vert_count[curr] += 1
                        break
                    if line == vertical_map[_vertical_found + 1 + x]:
                        vert_count[curr] += 1

            horiz_count = horizontal_found[horiz_count.index(max(horiz_count))] if horiz_count else 0
            vert_count = vertical_found[vert_count.index(max(vert_count))] if vert_count else 0
            results += vert_count if horiz_count < vert_count else horiz_count * 100

        return results

def get_reflections(puzzle):
    """ return largest reflection in puzzle based on mirror indexes """
    reflections = []
    indexes = []
    len_puzzle = len(puzzle)

    #display_map(puzzle)

    # find mirror positions
    last_line = puzzle[0]
    for y, line in enumerate(puzzle[1::]):
        if line == last_line:
            indexes.append(y + 1)
        last_line = line

    #print(f"Reflections: {indexes}")

    for index in indexes:
        count = 0
        valid = False
        i = index - 1
        for y, line in enumerate(puzzle[index - 1::-1]):
            if index + y >= len_puzzle:
                count += 1
                valid = True
                #print(f"Comparing {line} with empty line (count={count})")
            elif line == puzzle[index + y]:
                count += 1
                if i == 0:
                    valid = True
                #print(f"Comparing {line} with {puzzle[index + y]} (count={count})")
            else:
                #print(f"{y} ({i} - {valid}) => {count} {line} = (index={index})")
                break
            #print(f"{y} ({i} - {valid}) => {count} {line} = (index={index})")
            i -= 1
        if valid:
            reflections.append(count)

    #print(f"All reflections count: {reflections}")
    if not reflections:
        return 0
    return min(reflections)

def get_reflections_bruteforce(puzzle, smudge=None):
    """ return largest reflection in puzzle based on mirror indexes """
    base_reflection = get_reflections(puzzle)

    if smudge:
        pass

    for y, line in enumerate(puzzle):
        new_puzzle = [[x for x in y] for y in puzzle]
        for x, ch in enumerate(line):
            if ch == '.':
                new_puzzle[y][x] = '#'
            elif ch == '#':
                new_puzzle[y][x] = '.'
            new_new_puzzle = [''.join(line) for line in new_puzzle]
            new_reflection = get_reflections(new_new_puzzle)
            print(f"base={base_reflection} AND new={new_reflection}")
            if new_reflection and new_reflection != base_reflection:
                return new_reflection

    return 0


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
            horizontal = get_reflections(left)
            #print(f"Horizontal = {horizontal} ({horizontal*100})")
            #print("---")
            #print("Vertical:")
            vertical = get_reflections(vertical_map)
            #print(f"Vertical = {vertical}")
            if horizontal > vertical:
                #print(f"Chose horizontal = {horizontal*100} ------------------------------")
                results += horizontal * 100
                count += horizontal * 100
            else:
                #print(f"Chose vertical = {vertical} -----------------------------------")
                results += vertical
                count += vertical

            if (index+1) % 2 == 0:
                #print(f"======================= Result : {count} ==================================")
                count = 0

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
            horizontal = get_reflections_bruteforce(left)
            #print(f"Horizontal = {horizontal} ({horizontal*100})")
            #print("---")
            #print("Vertical:")
            vertical = get_reflections_bruteforce(vertical_map)
            #print(f"Vertical = {vertical}")
            if horizontal > vertical:
                #print(f"Chose horizontal = {horizontal*100} ------------------------------")
                results += horizontal * 100
                count += horizontal * 100
            else:
                #print(f"Chose vertical = {vertical} -----------------------------------")
                results += vertical
                count += vertical

            if (index+1) % 2 == 0:
                #print(f"======================= Result : {count} ==================================")
                count = 0

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
