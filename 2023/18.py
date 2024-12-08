#!/usr/bin/python

from pathlib import Path
import sys
import re
import shapely.geometry

import numpy as np

INPUT_PATH = str(Path(__file__).parent.resolve()) + '/inputs'
FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

INPUT_RE = re.compile(r"^([LRDU]) ([0-9]+) \(#([0-9a-f]+)\)$")

class Point:
    """ Defines a 2D point """
    def __init__(self, y, x=None):
        if x is None:
            self.y, self.x = y
        else:
            self.y = y
            self.x = x

    def __eq__(self, other):
        return (self.x, self.y) == (other.x, other.y)

    def __repr__(self):
        return f"{self.y},{self.x}"

    def __hash__(self):
        return hash(repr(self))

    def __add__(self, other):
        return Point(self.y + other.y, self.x + other.x)

    def __lt__(self, other):
        if self.y < other.y:
            return True
        if self.y == other.y:
            return self.x < other.x
        return False

    def __mul__(self, other):
        if isinstance(other, int):
            return Point(self.y * other, self.x * other)
        return Point(self.y * other.y, self.x * other.x)

DIRECTION_TO_STR = {
    Point(0, 1): "R",
    Point(1, 0): "D",
    Point(0, -1): "L",
    Point(-1, 0): "U"
}
STR_TO_DIRECTION = {r: l for l, r in DIRECTION_TO_STR.items()}

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        digpos = [Point(0, 0)]
        max_x = 0
        max_y = 0
        min_x = 0
        min_y = 0
        position = Point(0, 0)
        digpos_x = []
        digpos_y = []
        for _line in file:
            line = _line.rstrip()
            direction, depth, _ = INPUT_RE.findall(line)[0]
            direction = STR_TO_DIRECTION[direction]
            position += direction * int(depth)
            min_y = min(position.y, min_y)
            min_x = min(position.x, min_x)
            max_y = max(position.y, max_y)
            max_x = max(position.x, max_x)
            if position not in digpos:
                digpos.append(position)

        edges = [(p.y, p.x) for p in digpos] 
        polygon = shapely.geometry.Polygon(edges)
        return int(abs(polygon.area) + polygon.length/2 + 1)

def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        digpos = [Point(0,0)]
        max_x = 0
        max_y = 0
        min_x = 0
        min_y = 0
        position = Point(0, 0)
        for _line in file:
            line = _line.rstrip()
            _, _, color = INPUT_RE.findall(line)[0]
            depth = int(color[:-1:], 16)
            direction = STR_TO_DIRECTION[list(STR_TO_DIRECTION.keys())[int(color[-1])]]
            position += direction * int(depth)
            min_y = min(position.y, min_y)
            min_x = min(position.x, min_x)
            max_y = max(position.y, max_y)
            max_x = max(position.x, max_x)
            if position not in digpos:
                digpos.append(position)

        edges = [(p.y, p.x) for p in digpos] 
        polygon = shapely.geometry.Polygon(edges)
        return int(abs(polygon.area) + polygon.length/2 + 1)

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
