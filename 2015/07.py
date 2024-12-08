#!/usr/bin/python

from pathlib import Path
import sys
import operator

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

OPERATIONS = {"AND": operator.and_,
              "OR": operator.or_,
              "LSHIFT": operator.lshift,
              "RSHIFT": operator.rshift}

def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        wires = {k: 0 for k in []}
        for _line in file:
            line = _line.rstrip()
            left, right = line.split(" -> ")
            if "NOT" in left:
                wire = left.split("NOT ")[1]
                if right in wires:
                    wires[right] = ~wires[wire]
            else:
                operation_found = False
                for operation, func in OPERATIONS.items():
                    if operation in left:
                        operation_found = True
                        l, r = left.split(f" {operation} ")
                        if l.isdigit():
                            l = int(l)
                        elif l in wires:
                            l = wires[l]
                        else:
                            l = 0

                        if r.isdigit():
                            r = int(r)
                        elif r in wires:
                            r = wires[r]
                        else:
                            r = 0

                        if right in wires:
                            wires[right] = func(l, r)


                if not operation_found and right in wires:
                    wires[right] = int(left)

        return wires if 'a' not in wires else wires[a]



def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()


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
