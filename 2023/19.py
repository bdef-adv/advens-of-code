#!/usr/bin/python

from pathlib import Path
import sys

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
        val = int(val) + 1
        return f"{var}>{val}"
    else:
        var, val = condition.split('>')
        val = int(val) - 1
        return f"{var}<{val}"

def get_paths_to_A(workflows, conditions=[], workflow='in'):
    """ Get all the conditions possible to go from 'in' to 'A' """
    if workflow == "A":
        return conditions
    if workflow == "R":
        return []

    curr_workflow = workflows[workflow]
    steps = curr_workflow.split(',')

    last_destination = steps[-1]
    last_dest_index = len(steps) - 2

    neg_conditions = []
    curr_conditions = []
    for index, step in enumerate(steps[:-1]):
        condition, destination = step.split(':')
        neg_conditions.append(get_negative_condition(condition))
        curr_conditions.append(get_paths_to_A(workflows, conditions+[condition], destination))
    curr_conditions.append(get_paths_to_A(workflows, conditions + neg_conditions, last_destination))
    return curr_conditions



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
        
        all_conditions = []
        for _cond in get_paths_to_A(workflows):
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

        result = 0
        full_conditions = {"x": set(), "m": set(), "a": set(), "s": set()}
        for conditions in all_conditions:
            ranges = {
                "x": {"min": 1, "max": 4000},
                "m": {"min": 1, "max": 4000},
                "a": {"min": 1, "max": 4000},
                "s": {"min": 1, "max": 4000},
            }
            for cond in conditions:
                if '<' in cond:
                    var, val = cond.split('<')
                    ranges[var]["max"] = min(ranges[var]["max"], int(val)-1)
                if '>' in cond:
                    var, val = cond.split('>')
                    ranges[var]["min"] = max(ranges[var]["min"], int(val)+1)

            how_many_possibilities = ((ranges["x"]["max"] - ranges["x"]["min"]) *
                                      (ranges["m"]["max"] - ranges["m"]["min"]) *
                                      (ranges["a"]["max"] - ranges["a"]["min"]) *
                                      (ranges["s"]["max"] - ranges["s"]["min"]))
            print(conditions)
            for r, v in ranges.items():
                print(r, v)
            print(how_many_possibilities)

            for var in full_conditions.keys():
                full_conditions[var].add((ranges[var]["min"], ranges[var]["max"]))

            result += how_many_possibilities

        print(f"Result without filter: {result}")

        result = 0
        for var, l in full_conditions.items():
            res = 1
            for mini, maxi in l:
                res *= (maxi-mini)
            result += res

        return result


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
