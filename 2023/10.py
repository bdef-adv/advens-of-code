#!/usr/bin/python

from pathlib import Path
import sys

from shapely.geometry import Point, Polygon

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

"""
    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
"""

DIRECTIONS = {(-1, 0): '|',
              (0, 1): '-',
              (1, 0): '|',
              (0, 0): '',
              }

class Maze:
    def __init__(self):
        self.maze = []
        self.position = (0, 0)
        self.size = 0

    def append(self, line):
        self.maze.append(line)
        if 'S' in line:
            self.position = (self.size, line.find('S'))
        self.size += 1

    def get(self, tupl):
        y, x = tupl
        if y > self.size or x > len(self.maze[y]):
            return None
        return self.maze[y][x]

    def next_move(self, position, move_tuple):
        return tuple(map(sum, zip(position, move_tuple)))

    def get_next_direction(self, direction, curr_char):
        if curr_char is None:
            return None

        if direction == (-1, 0): # If we were going up '|'
            if curr_char == 'L':
                return None
            elif curr_char == 'J':
                return None
            elif curr_char == '7':
                return (0, -1)
            elif curr_char == 'F':
                return (0, 1)
            elif curr_char == '|':
                return direction
            elif curr_char == '-':
                return None
        elif direction == (1, 0): # If we were going down '|'
            if curr_char == 'L':
                return (0, 1)
            elif curr_char == 'J':
                return (0, -1)
            elif curr_char == '7':
                return None
            elif curr_char == 'F':
                return None
            elif curr_char == '|':
                return direction
            elif curr_char == '-':
                return None
        elif direction == (0, -1): # if we were going left '-', '7', 'J'
            if curr_char == 'L':
                return (-1, 0)
            elif curr_char == 'J':
                return None
            elif curr_char == '7':
                return None
            elif curr_char == 'F':
                return (1, 0)
            elif curr_char == '|':
                return None
            elif curr_char == '-':
                return direction
        elif direction == (0, 1): # if we were going right '-', 'F', 'L'
            if curr_char == 'L':
                return None
            elif curr_char == 'J':
                return (-1, 0)
            elif curr_char == '7':
                return (1, 0)
            elif curr_char == 'F':
                return None
            elif curr_char == '|':
                return None
            elif curr_char == '-':
                return direction
            

    def get_max_moves(self, direction):
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
        polygon = Polygon(paths)
        _y, _x = point
        point = Point(_y, _x)
        return polygon.contains(point)

    def count_enclosed_by_path(self, paths):
        points = set()

        n_paths = [v[0] for v in paths]

        for y, rows in enumerate(self.maze):
            for x, ch in enumerate(rows):
                point = (y, x)
                if point in n_paths:
                    continue
                if self.is_point_in_maze(point, n_paths):
                    points.add(point)
        return len(points)


def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        maze = Maze()
        for y, _line in enumerate(file):
            line = _line.rstrip()
            maze.append(line)

        steps = 0
        max_steps = 0
        proper_way = ""

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
        for y, _line in enumerate(file):
            line = _line.rstrip()
            maze.append(line)

        steps = 0
        max_steps = 0
        proper_way = ""

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
    print(solution_part1(f"input.{FILENAME_TRUNC}.test.txt"))

    print("Result:")
    print(solution_part1(f"input.{FILENAME_TRUNC}.txt"))

    print("--- Part Two ---")
    print("Test result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.test.txt"))

    print("Result:")
    print(solution_part2(f"input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
