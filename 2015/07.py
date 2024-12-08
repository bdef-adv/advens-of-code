#!/usr/bin/python

from pathlib import Path
import sys
import operator
from collections import defaultdict
import re

FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

OPERATIONS = {"AND": operator.and_,
              "OR": operator.or_,
              "LSHIFT": operator.lshift,
              "RSHIFT": operator.rshift}

RESULTS = {}

def calculate(wires, key):
    """ recursive function to calculate results """
    operation, left, right = wires[key]

    #print(f"\rcalculating {key} => {left} {operation} {right}", end="")

    if operation == "IS":
        if left.isdigit():
            RESULTS[left] = int(left)
            return int(left)
        if left in RESULTS:
            return RESULTS[left]

        return calculate(wires, left)

    if operation == "NOT":
        if left.isdigit():
            return ~int(left)
        if left in RESULTS:
            return ~RESULTS[left]

        return ~calculate(wires, left)

    if operation in OPERATIONS:
        l, r = 0, 0
        if left.isdigit():
            l = int(left)
        elif left in RESULTS:
            l = RESULTS[left]
        else:
            l = calculate(wires, left)
        if right.isdigit():
            r = int(right)
        elif right in RESULTS:
            r = RESULTS[right]
        else:
            r = calculate(wires, right)

        res = OPERATIONS[operation](l, r)
        RESULTS[key] = res 
        return res



def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        wires = defaultdict(list)
        for _line in file:
            line = _line.rstrip()
            left, right = line.split(" -> ")
            if "NOT" in left:
                wire = left.split("NOT ")[1]
                wires[right] = ("NOT", wire, None)
            else:
                assignment = re.compile(r"^([\d\w]+)$")
                assignment = assignment.findall(left)
                if assignment:
                    wires[right] = ("IS", assignment[0], None)
                    continue

                operations = re.compile(r"^([a-z\d]+) ([A-Z]+) ([a-z\d]+)$")
                operations = operations.findall(left)
                if not operations:
                    continue

                l, operation, r = operations[0]
                wires[right] = (operation, l, r)

            #print(right, '=', wires[right])

        if 'a' not in wires:
            return wires

        part1 = calculate({k: wires[k] for k in sorted(wires)}, "a")

        print(wires['b'])
        wires['b'] = wires['a'] #("IS", str(part1), None)
        print(wires['b'])
        global RESULTS
        RESULTS = []
        part2 = calculate({k: wires[k] for k in sorted(wires)}, "a")
        return (part1, part2)



if __name__ == "__main__":
    print("--- Part One ---")
    print("Test result:")
    print(solution_part1(f"input.{FILENAME_TRUNC}.test.txt"))

    print("Result:")
    print(solution_part1(f"input.{FILENAME_TRUNC}.txt"))
