#!/usr/bin/python
""" day 8 """
from pathlib import Path
import sys
from math import gcd, floor

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        instruction = ""
        positions = {}
        for _line in file:
            line = _line.rstrip()
            if line and '=' not in line:
                instruction = line
            elif line:
                left, right = line.split(' = ')
                lright, rright = right.replace('(', '').replace(')', '').split(', ')
                positions[left] = (lright, rright)

        steps = 0
        position = "AAA"
        while position != 'ZZZ':
            for ch in instruction:
                next_pos = positions[position][0] if ch == 'L' else positions[position][1]
                position = next_pos
                steps += 1
                if position == "ZZZ":
                    break

        return steps


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        instruction = ""
        positions = {}
        starting_nodes = []
        for _line in file:
            line = _line.rstrip()
            if line and '=' not in line:
                instruction = line
            elif line:
                left, right = line.split(' = ')
                lright, rright = right.replace('(', '').replace(')', '').split(', ')
                positions[left] = (lright, rright)
                if left.endswith('A'):
                    starting_nodes.append(left)

        nodes = starting_nodes
        result = 1
        for node in nodes:
            steps = 0
            position = node
            while position[-1] != "Z":
                for ch in instruction:
                    steps += 1
                    position = positions[position][0] if ch == 'L' else positions[position][1]
                    if position[-1] == 'Z':
                        result = floor(result * steps / gcd(result, steps))
                        break

        return result


if __name__ == "__main__":
    print("--- Part One ---")
    print("Test result:")
    #print(solution_part1(f"input.{FILENAME_TRUNC}.test.txt"))

    print("Result:")
    print(solution_part1(f"input.{FILENAME_TRUNC}.txt"))

    print("--- Part Two ---")
    print("Test result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.test.txt"))

    print("Result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
