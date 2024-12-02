#!/usr/bin/python

import sys
import hashlib
import threading

from pathlib import Path

INPUT_PATH = str(Path(__file__).parent.resolve()) + '/inputs'
FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

SPLIT = 200000
THREADS_NUMBER = 5

START_2 = 1000000
SPLIT_2 = 1000000
THREADS_NUMBER_2 = 20

def find_proper_hash(string, nstart, nrange, result=None, nzeroes=5):
    """ Returns the first number to have a md5 hash starting with nzeroes '0'
    """
    for i in range(nstart, nrange):
        if hashlib.md5(f"{string}{i}".encode()).hexdigest().startswith(nzeroes*"0"):
            if not result["result"]:
                result["result"] = i
            else:
                result["result"] = min(result["result"], i)
            return i

        if result["result"]:
            return result["result"]

def solution_part1_threaded(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()
            result = {"result": None}
            threads = [None] * THREADS_NUMBER
            for i in range(THREADS_NUMBER):
                threads[i] = threading.Thread(target=find_proper_hash, args=(line, SPLIT*i, SPLIT*i+SPLIT, result))
                threads[i].start()

            for i in range(THREADS_NUMBER):
                threads[i].join()

            return result["result"]

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()
            for i in range(0, SPLIT*THREADS_NUMBER):
                if hashlib.md5(f"{line}{i}".encode()).hexdigest().startswith(5*"0"):
                    return i
            return None


def solution_part2_threaded(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()
            result = {"result": []}
            threads = [None] * THREADS_NUMBER_2
            for i in range(THREADS_NUMBER_2):
                threads[i] = threading.Thread(target=find_proper_hash,
                                              args=(line, START_2+ SPLIT_2*i, START_2+SPLIT_2*i+SPLIT_2, result, 6))
                threads[i].start()

            for i in range(THREADS_NUMBER_2):
                threads[i].join()

            return result["result"]


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()
            for i in range(START_2, START_2 + SPLIT_2*THREADS_NUMBER_2):
                if hashlib.md5(f"{line}{i}".encode()).hexdigest().startswith(6*"0"):
                    return i
            return None


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
