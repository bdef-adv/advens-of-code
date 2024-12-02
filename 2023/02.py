#!/usr/bin/python
""" day 2 """
from pathlib import Path
import sys

INPUT_PATH = str(Path(__file__).parent.resolve()) + '/inputs'
FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        final_count = 0
        for _line in file:
            line = _line.rstrip()
            game, data = line.split(': ')
            game_number = int(game.split(' ')[1])

            sets = data.split('; ')
            balls = [b.split(', ') for b in sets]

            all_valid = True
            for draw in balls:
                count = {"red": 0, "green": 0, "blue": 0}
                for ball in draw:
                    n, color = ball.split(' ')
                    count[color] += int(n)
                if count["red"] > 12 or count["green"] > 13 or count["blue"] > 14:
                    all_valid = False
                    break
            if all_valid:
                final_count += game_number

    return final_count


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        power = 0
        for _line in file:
            line = _line.rstrip()
            _, data = line.split(': ')

            sets = data.split('; ')
            balls = [b.split(', ') for b in sets]

            min_balls = {"red": 0, "green": 0, "blue": 0}

            for draw in balls:
                count = {"red": 0, "green": 0, "blue": 0}
                for ball in draw:
                    n, color = ball.split(' ')
                    count[color] += int(n)

                for color, value in count.items():
                    min_balls[color] = value if value > min_balls[color] else min_balls[color]

            power += min_balls["red"] * min_balls["green"] * min_balls["blue"]

    return power


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
