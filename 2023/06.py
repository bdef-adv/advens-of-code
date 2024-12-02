#!/usr/bin/python
""" day 6 """
from pathlib import Path
import sys

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        time = []
        distance = []
        for _line in file:
            line = _line.rstrip()
            if line.startswith("Time:"):
                time = list(map(int, line.split("Time:")[1].split()))
            if line.startswith("Distance:"):
                distance = list(map(int, line.split("Distance:")[1].split()))
        print(time, distance)

        won = []
        for x, record in enumerate(time):
            current_wins = []
            dist = distance[x]
            for holding in range(record):
                time_left = record - holding
                curr_distance = holding * time_left
                #print(f"Holding {holding} secs to beat {record} (distance = {dist}), moved {curr_distance}")
                if curr_distance > dist:
                    current_wins.append(holding)
            won.append(len(current_wins))

        result = won[0]
        for w in won[1::]:
            result *= w
        return result


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        time = []
        distance = []
        for _line in file:
            line = _line.rstrip()
            if line.startswith("Time:"):
                time = int(''.join(line.split("Time:")[1].split()))
            if line.startswith("Distance:"):
                distance = int(''.join(line.split("Distance:")[1].split()))
        print(time, distance)

        won = []
        record = time
        current_wins = []
        dist = distance
        for holding in range(record):
            time_left = record - holding
            curr_distance = holding * time_left
            #print(f"Holding {holding} secs to beat {record} (distance = {dist}), moved {curr_distance}")
            if curr_distance > dist:
                current_wins.append(holding)
        won.append(len(current_wins))

        result = won[0]
        for w in won[1::]:
            result *= w
        return result


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
