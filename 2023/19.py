#!/usr/bin/python

from pathlib import Path
import sys

from collections import defaultdict
from copy import deepcopy

INPUT_PATH = str(Path(__file__).parent.resolve()) + '/inputs'
FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def get_path(workflows, part, workflow="in"):
    if workflow in ["A", "R"]:
        return workflow

    workflow = workflows[workflow]
    steps = workflow.split(',')

    last_destination = steps[-1]
    last_dest_index = len(steps) - 2
    for index, step in enumerate(steps[:-1]):
        condition, destination = step.split(':')
        if '<' in condition:
            var, val = condition.split('<')
            if part[var] < int(val):
                return get_path(workflows, part, destination)
            elif index == last_dest_index:
                return get_path(workflows, part, last_destination)
        elif '>' in condition:
            var, val = condition.split('>')
            if part[var] > int(val):
                return get_path(workflows, part, destination)
            elif index == last_dest_index:
                return get_path(workflows, part, last_destination)

def get_negative_condition(condition):
    """ for x<1345 return x>1344 and for x>123 return x<124 """
    if '<' in condition:
        var, val = condition.split('<')
        val = int(val) - 1
        return f"{var}>{val}"
    else:
        var, val = condition.split('>')
        val = int(val) + 1
        return f"{var}<{val}"

def range_from_conditions(conditions, ranges):
    new_range = deepcopy(ranges)
    for condition in conditions:
        if '<' in condition:
            var, val = condition.split('<')
            new_range[var] = sorted([ranges[var][0], min(ranges[var][1], int(val)-1)])
        if '>' in condition:
            var, val = condition.split('>')
            new_range[var] = sorted([max(ranges[var][0], int(val)+1), ranges[var][1]])
    return new_range

def _calculate_range(ran, start):
    diffs = []
    for var, val in ran.items():
        curr = val[1] - val[0] + 1
        if curr != 4000:
            diffs.append(curr)
    return start // 4000 * sum(diffs)

def calculate_range(ran, start):
    return ((ran["x"][1] - ran["x"][0] + 1) *
            (ran["m"][1] - ran["m"][0] + 1) *
            (ran["a"][1] - ran["a"][0] + 1) *
            (ran["s"][1] - ran["s"][0] + 1))

def get_all_paths(workflows, workflow='in', start=4000*4000*4000*4000, ranges={"x": [1, 4000], "m": [1, 4000], "a": [1, 4000], "s": [1, 4000]}, indent=0):
    """ Get all the conditions possible to go from 'in' to 'A' """
    if workflow in ["A", "R"]:
        return {workflow: start}

    steps = workflows[workflow].split(',')
    last_destination = steps[-1]

    remainder = start

    nodes = defaultdict(int)
    conditions = defaultdict(list)
    for index, step in enumerate(steps[:-1]):
        condition, destination = step.split(':')
        conditions[destination].append(condition)

    print(f"{indent*'  ' }// Ranges={ranges}")

    for destination, conditions in conditions.items():
        curr_range = range_from_conditions(conditions, ranges)
        diff = calculate_range(curr_range, remainder)
        print(f"{indent*'  '}// {workflow} Calculating {destination} with {conditions} => {curr_range} (diff={diff}) current remainder={remainder} next={remainder-diff}")
        nodes[destination] = (diff, curr_range)
        remainder -= diff


    print(f"{indent*'  '}{workflow} [{remainder}] {last_destination}")
    for dest, val in nodes.items():
        val, ran = val
        print(f"{indent*'  '}{workflow} [{val}] {dest}")
        nodes = dict(nodes, **get_all_paths(workflows, dest, val, ran, indent+1))

    nodes = dict(nodes, **get_all_paths(workflows, last_destination, remainder, ranges, indent+1))

    nodes[last_destination] = remainder
    return nodes


def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        workflows = {}
        parts = []
        reading_parts = False
        accepted = []
        for _line in file:
            line = _line.rstrip()
            if not line:
                reading_parts = True
                continue
            if not reading_parts:
                name, workflow = line.split('{')
                workflows[name] = workflow.rstrip('}')
            else:
                part = line.lstrip('{}').rstrip('}').split(',')
                part = {p.split('=')[0]: int(p.split('=')[1]) for p in part}
                parts.append(part)
                if get_path(workflows, part) == "A":
                    accepted.append(part)

        result = 0
        for parts in accepted:
            for name, value in parts.items():
                result += value
        return result


def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        workflows = {}
        reading_parts = False
        accepted = []
        for _line in file:
            line = _line.rstrip()
            if not line:
                reading_parts = True
                continue
            if not reading_parts:
                name, workflow = line.split('{')
                workflows[name] = workflow.rstrip('}')
            else:
                break
        
        destinations = get_all_paths(workflows)
        print(destinations)
        #for destination, start in destinations.items():
        #    print(f"Calculating {destination} with start={start}")
        #    destinations = get_all_paths(workflows, destination, start)


"""
        all_conditions = []
        for _cond in get_paths_to_A(workflows):
            print(_cond)
            for conditions in _cond:
                if conditions and isinstance(conditions[0], list):
                    for cond in conditions:
                        if cond and isinstance(cond[0], list):
                            for con in cond:
                                if con:
                                    all_conditions.append(con)
                        elif cond:
                            all_conditions.append(cond)
                elif conditions:
                    all_conditions.append(conditions)
    
        full_conditions = defaultdict(list)
        for conditions in all_conditions:
            ranges = {
                "x": [1, 4000],
                "m": [1, 4000],
                "a": [1, 4000],
                "s": [1, 4000],
            }
            for cond in conditions:
                if '<' in cond:
                    var, val = cond.split('<')
                    ranges[var][1] = min(ranges[var][1], int(val)-1)
                if '>' in cond:
                    var, val = cond.split('>')
                    ranges[var][0] = max(ranges[var][0], int(val)+1)

            ranges = {k: tuple(v) for k, v in ranges.items()}
            print(ranges)

            full_conditions[ranges["x"]].append([ranges["m"], ranges["a"], ranges["s"]])
            #full_conditions[ranges["m"]].append([ranges["x"], ranges["a"], ranges["s"]])
            #full_conditions[ranges["a"]].append([ranges["x"], ranges["m"], ranges["s"]])
            #full_conditions[ranges["s"]].append([ranges["x"], ranges["m"], ranges["a"]])

        print(full_conditions)
        result = 0
        for var, l in full_conditions.items():
            for ranges in l:
                cur_range = var[1] - var[0]
                for ran in ranges:
                    print(var, ran)
                    cur_range *= ran[1] - ran[0]
                result += cur_range
            print(cur_range, result)
"""
        #return result


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
    #print(solution_part2(f"{INPUT_PATH}/input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
