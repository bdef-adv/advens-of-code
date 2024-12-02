#!/usr/bin/python
"""day10
"""
from pathlib import Path
import sys

from shapely.geometry import Point, Polygon

INPUT_PATH = str(Path(__file__).parent.resolve()) + '/inputs'
FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""


class Maze:
    """ maze class """
    def __init__(self):
        self.maze = []
        self.position = (0, 0)
        self.size = 0

    def append(self, line):
        """ add line to maze """
        self.maze.append(line)
        if 'S' in line:
            self.position = (self.size, line.find('S'))
        self.size += 1

    def get(self, position):
        """ get character at position """
        y, x = position
        if y > self.size or x > len(self.maze[y]):
            return None
        return self.maze[y][x]

    def next_move(self, position, move_tuple):
        """ Get next move by adding position and move_tuple values """
        return tuple(map(sum, zip(position, move_tuple)))

    def get_next_direction(self, direction, curr_char):
        """ Get next direction for coursing through the maze """
        if curr_char is None:
            return None

        if direction == (-1, 0): # If we were going up '|'
            if curr_char == '|':
                return direction
            if curr_char in '-LJ':
                return None
            if curr_char == '7':
                return (0, -1)
            if curr_char == 'F':
                return (0, 1)
        elif direction == (0, 1): # if we were going right '-', 'F', 'L'
            if curr_char in '|LF':
                return None
            if curr_char == '-':
                return direction
            if curr_char == 'J':
                return (-1, 0)
            if curr_char == '7':
                return (1, 0)
        elif direction == (1, 0): # If we were going down '|'
            if curr_char == 'L':
                return (0, 1)
            if curr_char == 'J':
                return (0, -1)
            if curr_char in '7F-':
                return None
            if curr_char == '|':
                return direction
        elif direction == (0, -1): # if we were going left '-', '7', 'J'
            if curr_char == 'L':
                return (-1, 0)
            if curr_char == 'J7|':
                return None
            if curr_char == 'F':
                return (1, 0)
            if curr_char == '-':
                return direction


    def get_max_moves(self, direction):
        """ get max moves taken when going down a direction """
        curr_char = 'S'
        steps = 0
        next_pos = self.position

        while curr_char != '.':
            next_pos = self.next_move(next_pos, direction)
            curr_char = self.get(next_pos)
            steps += 1

            direction = self.get_next_direction(direction, curr_char)
            if direction is None:
                return steps

        return steps


    def get_all_paths(self, direction):
        """ get all paths along the directio """
        curr_char = 'S'
        steps = [(self.position, direction)]
        next_pos = self.position

        while curr_char != '.':
            next_pos = self.next_move(next_pos, direction)
            curr_char = self.get(next_pos)
            steps.append((next_pos, direction))

            direction = self.get_next_direction(direction, curr_char)
            if direction is None:
                return steps

    def is_point_in_maze(self, point, paths):
        """ Method to check if the point is inside the maze or not """
        if point in paths:
            return False
        polygon = Polygon(paths)
        _y, _x = point
        point = Point(_y, _x)
        return polygon.contains(point)

    def count_enclosed_by_path(self, paths):
        """ Count every tile enclosed in maze """
        points = set()

        n_paths = [v[0] for v in paths]

        for y, rows in enumerate(self.maze):
            for x, _ in enumerate(rows):
                point = (y, x)
                #if point in n_paths:
                #    continue
                if self.is_point_in_maze(point, n_paths):
                    points.add(point)
        return len(points)


def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        maze = Maze()
        for _line in file:
            line = _line.rstrip()
            maze.append(line)

        answer = list(filter(lambda v: v % 2 == 0, [
            maze.get_max_moves((1, 0)),
            maze.get_max_moves((-1, 0)),
            maze.get_max_moves((0, 1)),
            maze.get_max_moves((0, -1))
        ]))

        return answer[0] // 2



def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        maze = Maze()
        for _line in file:
            line = _line.rstrip()
            maze.append(line)

        paths = list(filter(lambda v: len(v) % 2 != 0, [
            maze.get_all_paths((1, 0)),
            maze.get_all_paths((-1, 0)),
            maze.get_all_paths((0, 1)),
            maze.get_all_paths((0, -1))
        ]))[0]

        return maze.count_enclosed_by_path(paths)


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
