#!/usr/bin/python

from pathlib import Path
import sys

INPUT_PATH = str(Path(__file__).parent.resolve()) + '/inputs'
FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""


class Submarine:
    def __init__(self, x=0, y=0):
        self.x = x
        self.y = y
        self.horizontal = 0
        self.vertical = 0

    def forward(self, val):
        self.x += val
        self.horizontal += val

    def down(self, val):
        self.y += val
        self.vertical += val


class Submarine2:
    def __init__(self, x=0, y=0, aim=0):
        self.x = x
        self.y = y
        self.aim = 0
        self.horizontal = 0
        self.vertical = 0

    def forward(self, val):
        self.x += val
        self.horizontal += val
        self.y += self.aim * val

    def down(self, val):
        self.aim += val

    def up(self, val):
        self.aim -= val


def solution_part1(filename):
    with open(filename, "r") as file:
        submarine = Submarine()

        for _line in file:
            line = _line.rstrip()
            instruction, val = line.split()
            val = int(val)
            
            if instruction == "forward":
                submarine.forward(val)
            elif instruction == "down":
                submarine.down(val)
            elif instruction == "up":
                submarine.down(-val)

        return submarine.horizontal*submarine.vertical


def solution_part2(filename):
    with open(filename, "r") as file:
        submarine = Submarine2()

        for _line in file:
            line = _line.rstrip()
            instruction, val = line.split()
            val = int(val)
            
            if instruction == "forward":
                submarine.forward(val)
            elif instruction == "down":
                submarine.down(val)
            elif instruction == "up":
                submarine.up(val)

        return submarine.horizontal*submarine.y


if __name__ == "__main__":
    print("--- Part One ---")
    print("Test result:")
    print(solution_part1(f"{INPUT_PATH}/input.{FILENAME_TRUNC}.test.txt"))

    print("Result:")
    print(solution_part1(f"{INPUT_PATH}/input.{FILENAME_TRUNC}.txt"))

    print("--- Part Two ---")
    print("Test result:")
    print(solution_part2(f"{INPUT_PATH}/input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.test.txt"))

    print("Result:")
    print(solution_part2(f"{INPUT_PATH}/input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
