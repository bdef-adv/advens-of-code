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

def dedupe_adjacent(alist):
    alist = list(alist)
    for i in range(len(alist) - 1, 0, -1):
        if alist[i] == alist[i-1]:
            del alist[i]
    return alist

POLYGON = None

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

    def in_map(self):
        """ Is point in map? """
        global POLYGON
        return POLYGON.contains(shapely.geometry.Point(self.y, self.x))

    def is_corner_paths(self, paths):
        """ count number of '#' adjacent to current position """
        if self not in paths:
            return False

        right = Point(self.y, self.x + 1) in paths
        left = Point(self.y, self.x - 1) in paths
        top = Point(self.y - 1, self.x) in paths
        down = Point(self.y + 1, self.x) in paths

        return ((right and down) or (right and top) or
                (down and left) or (down and right) or
                (top and left) or (top and right) or
                (left and down) or (left and top))

    def is_corner(self, puzzle):
        """ count number of '#' adjacent to current position """
        if puzzle[self.y][self.x] == '.':
            return 0
        edges = 0
        left, right, down, top = False, False, False, False
        # right
        if self.x + 1 < len(puzzle[0]):
            right = puzzle[self.y][self.x + 1] == '#'
            edges += int(puzzle[self.y][self.x + 1] == '#')
        # left
        if self.x - 1 >= 0:
            left = puzzle[self.y][self.x - 1] == '#'
            edges += int(puzzle[self.y][self.x - 1] == '#')
        # top
        if self.y - 1 >= 0:
            top = puzzle[self.y - 1][self.x] == '#'
            edges += int(puzzle[self.y - 1][self.x] == '#')
        # down
        if self.y + 1 < len(puzzle):
            down = puzzle[self.y + 1][self.x] == '#'
            edges += int(puzzle[self.y + 1][self.x] == '#')

        return ((right and down) or (right and top) or
                (down and left) or (down and right) or
                (top and left) or (top and right) or 
                (left and down) or (left and top) or
                (self.x - 1 < 0 and edges == 1))

DIRECTION_TO_STR = {
    Point(0, 1): "R",
    Point(1, 0): "D",
    Point(0, -1): "L",
    Point(-1, 0): "U"
}
STR_TO_DIRECTION = {r: l for l, r in DIRECTION_TO_STR.items()}

def display_map(puzzle):
    """ display 2d map """
    print("Map:")
    for l in puzzle:
        print(l)

def get_edges_paths(paths):
    """ get edges """
    edges = []
    for point in paths:
        if point.is_corner_paths(paths):
            edges.append((point.y, point.x))
    return edges

def count_all_positions(paths, maximum, minimum):
    """ count all without having to draw a map """  
    min_y, min_x = minimum
    max_y, max_x = maximum
    count = 0
    for y in range(min_y - 1, max_y + 3):
        for x in range(min_x - 1, max_x + 3):
            if Point(y, x) not in paths:
                if Point(y, x).in_map():
                    count += 1
            else:
                count += 1
    return count

def get_area(points):
    """ det(A)=ad−bc
        | a b |
        | c d |
        | p1.y p2.y |
        | p1.x p2.x |
    """
    area = 0
    last_point = Point(points[0])
    for point in points[1::]:
        point = Point(point)
        area += last_point.x * point.y - last_point.y * point.x
    return area // 2

def PolyArea(x,y):
    return 0.5*np.abs(np.dot(x,np.roll(y,1))-np.dot(y,np.roll(x,1)))

def polygon_area(x,y):
    correction = x[-1] * y[0] - y[-1]* x[0]
    main_area = np.dot(x[:-1], y[1:]) - np.dot(y[:-1], x[1:])
    return 0.5*np.abs(main_area + correction)

def solution_part1(filename):
    """ PART 1
    """
    global POLYGON
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
            for _ in range(int(depth)):
                position += direction #* int(depth)
                min_y = min(position.y, min_y)
                min_x = min(position.x, min_x)
                max_y = max(position.y, max_y)
                max_x = max(position.x, max_x)
                if position not in digpos:
                    digpos.append(position)

        edges = get_edges_paths(digpos)
        POLYGON = shapely.geometry.Polygon(edges)
        print(len(digpos), len(POLYGON.exterior.coords), get_area(POLYGON.exterior.coords), POLYGON.area)
        return count_all_positions(digpos, (max_y, max_x), (min_y, min_x))

def solution_part2(filename):
    """ PART 2
    """
    global POLYGON
    with open(filename, "r", encoding="utf-8") as file:
        s = 800000
        digpos = [Point(s, s)]
        max_x = 0
        max_y = 0
        min_x = 0
        min_y = 0
        position = Point(s, s)
        for _line in file:
            line = _line.rstrip()
            _, _, color = INPUT_RE.findall(line)[0]
            depth = int(color[:-1:], 16)
            direction = STR_TO_DIRECTION[list(STR_TO_DIRECTION.keys())[int(color[-1])]]
            for _ in range(int(depth)):
                position += direction
                min_y = min(position.y, min_y)
                min_x = min(position.x, min_x)
                max_y = max(position.y, max_y)
                max_x = max(position.x, max_x)
                if position not in digpos:
                    digpos.append(position)

        edges = get_edges_paths(digpos)
        POLYGON = shapely.geometry.Polygon(edges)
        return count_all_positions(digpos, (max_y, max_x), (min_y, min_x))


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
