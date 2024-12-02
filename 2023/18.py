#!/usr/bin/python

from pathlib import Path
import sys
import re
import shapely.geometry

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

class Point:
    """ Defines a 2D point """
    def __init__(self, y, x):
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

    def in_map(self, paths):
        if (self.y, self.x) in paths:
            return False
        polygon = shapely.geometry.Polygon(paths)
        point = shapely.geometry.Point(self.y, self.x)
        return polygon.contains(point)

    def in_map_puzzle(self, puzzle):
        """ how many edges on one side """
        edges_after_point = 0
        for x, ch in enumerate(puzzle[self.y]):
            if x < self.x or ch != '#':
                continue
            if Point(self.y, x).is_wall(puzzle):
                edges_after_point += 1

        #print(self, edges_after_point)
        dedupe_right = dedupe_adjacent(puzzle[self.y][self.x+1:])
        dedupe_left = dedupe_adjacent(puzzle[self.y][:self.x:])
        gaps_right = puzzle[self.y][self.x+1:''.join(puzzle[self.y][self.x+1::]).rfind('#')].count('.') == 1
        #print(f"{self} => edges={edges_after_point}, gaps_right={gaps_right}, dedupe={dedupe_left},{dedupe_right}")
        return ((edges_after_point % 2 != 0) or
                (dedupe_right.count('#') % 2 != 0 and (dedupe_left.count('#') == dedupe_right.count('#'))))

    def is_wall(self, puzzle):
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

        return ((right and down) or
                (down and left) or (down and right) or
                (top and left) or (top and right) or 
                (left and down) or (left and top) or
                (top and down))

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

        if self.x in [18, 19, 20, 21] and self.y == 0:
            print(f"Point {self}, {edges} ({left}, {right}, {down}, {top})")

        return ((right and down) or (right and top) or
                (down and left) or (down and right) or
                (top and left) or (top and right) or 
                (left and down) or (left and top) or
                (self.x - 1 < 0 and edges == 1))
    
    def is_corner_paths(self, paths):
        """ count number of '#' adjacent to current position """
        if self not in paths:
            return 0
        right = Point(self.y, self.x + 1) in paths
        left = Point(self.y, self.x - 1) in paths
        top = Point(self.y - 1, self.x) in paths
        down = Point(self.y + 1, self.x) in paths

        return ((right and down) or (right and top) or
                (down and left) or (down and right) or
                (top and left) or (top and right) or 
                (left and down) or (left and top))

DIRECTION_TO_STR = {
    Point(0, 1): "R",
    Point(0, -1): "L",
    Point(1, 0): "D",
    Point(-1, 0): "U"
}
STR_TO_DIRECTION = {r: l for l, r in DIRECTION_TO_STR.items()}

def display_map(puzzle, beams=None):
    """ display 2d map """
    print("Map:")
    for l in puzzle:
        print(l)
    return None

def count_holes(puzzle):
    """ Count '#' chars in puzzle """
    c = 0
    for l in puzzle:
        c += l.count('#')
    return c

def dig_that_shit_homes(puzzle, paths):
    """ Add all dig points inside """
    new_puzzle = puzzle
    for y, line in enumerate(new_puzzle):
        new_puzzle[y] = list(new_puzzle[y])
        for x, ch in enumerate(line):
            if Point(y, x).in_map(paths):
                new_puzzle[y][x] = '#'
        new_puzzle[y] = ''.join(new_puzzle[y])
    return new_puzzle

def get_edges(puzzle, paths):
    """ get edges """
    edges = []
    for point in paths:
        if point.is_corner(puzzle):
            edges.append((point.y, point.x))
    return edges

def get_edges_paths(paths):
    """ get edges """
    edges = []
    for point in paths:
        if point.is_corner_paths(paths):
            edges.append((point.y, point.x))
    return edges

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        s = 150
        digpos = [Point(s, s)]
        max_x = 0
        max_y = 0
        position = Point(s, s)
        for _line in file:
            line = _line.rstrip()
            direction, depth, color = INPUT_RE.findall(line)[0]
            direction = STR_TO_DIRECTION[direction]
            for i in range(int(depth)):
                position += direction
                max_y = max(position.y, max_y)
                max_x = max(position.x, max_x)
                if position not in digpos:
                    digpos.append(position)

        #digmap = []
        #for y in range(max_y + 1):
        #    digmap.append("")
        #    for x in range(max_x + 1):
        #        digmap[y] += "#" if Point(y, x) in digpos else '.'
        #    digmap[y] += '.'

        #display_map(digmap)
        #edges = get_edges(digmap, digpos)
        edges = get_edges_paths(digpos)
        digmap = dig_that_shit_homes(digmap, edges)
        #display_map(digmap)
        return count_holes(digmap)
            

DIRECTIONS = ["R", "D", "L", "U"]

def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        s = 800000
        digpos = [Point(s, s)]
        max_x = 0
        max_y = 0
        position = Point(s, s)
        for _line in file:
            line = _line.rstrip()
            _, _, color = INPUT_RE.findall(line)[0]
            depth = int(color[:-1:], 16)
            direction = STR_TO_DIRECTION[DIRECTIONS[int(color[-1])]]
            for i in range(int(depth)):
                position += direction
                max_y = max(position.y, max_y)
                max_x = max(position.x, max_x)
                if position not in digpos:
                    digpos.append(position)

        edges = get_edges_paths(digpos)
        digmap = dig_that_shit_homes_part2(edges)
        return count_holes(digmap)


if __name__ == "__main__":
    print("--- Part One ---")
    print("Test result:")
    print(solution_part1(f"{INPUT_PATH}/input.{FILENAME_TRUNC}.test.txt"))

    print("Result:")
    #print(solution_part1(f"{INPUT_PATH}/input.{FILENAME_TRUNC}.txt"))

    print("--- Part Two ---")
    print("Test result:")
    print(solution_part2(f"{INPUT_PATH}/input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.test.txt"))

    print("Result:")
    #print(solution_part2(f"{INPUT_PATH}/input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
