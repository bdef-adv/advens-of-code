#!/usr/bin/python

from pathlib import Path
import sys
from collections import defaultdict
from operator import add

INPUT_PATH = str(Path(__file__).parent.resolve()) + '/inputs'
FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

PULSES = {"low": False, "high": True}
PULSES_REV = {False: "low", True: "high"}

MODULES = defaultdict(bool)
CONJUNCTIONS = {}
HISTORY = []

def send_pulse(prefix, module, destination):
    global HISTORY
    global MODULES
    counts = {False: 0, True: 0}
    if prefix == '&':
        count_highs = len(HISTORY)
        for mod, pul, des in HISTORY:
            if pul:
                count_highs -= 1
        next_pulse = True
        if not count_highs:
            next_pulse = False

        for dest in destination:
            print(f"{module} -{PULSES_REV[next_pulse]}-> {dest}")
            MODULES[dest] = next_pulse
            counts[next_pulse] += 1

        #print(HISTORY)
        #for mod, pul, des in HISTORY:
        #    print(f"{mod} -{PULSES_REV[next_pulse]}-> {des}")
        #    MODULES[des] = next_pulse
        #    counts[next_pulse] += 1

        #HISTORY = []

        #for dest in destination:
        #    print(f"{module} -{PULSES_REV[not next_pulse]}-> {dest}")
        #    MODULES[dest] = not next_pulse
        #    counts[not next_pulse] += 1

        return counts

    if prefix == 'b':
        print(f"button {PULSES_REV[MODULES[module]]}-> broadcaster")
        counts[MODULES[module]] += 1

    for dest in destination:
        curr_pulse = MODULES[module]
        if prefix == 'b':
            print(f"broadcaster -{PULSES_REV[curr_pulse]}-> {dest}")
            MODULES[dest] = curr_pulse
            counts[curr_pulse] += 1
        elif prefix == '%':
            if curr_pulse:
                HISTORY.append((module, curr_pulse, dest))
                counts[curr_pulse] += 1
                continue
            next_pulse = not MODULES[dest]
            print(f"{module} -{PULSES_REV[next_pulse]}-> {dest}")
            MODULES[dest] = next_pulse
            HISTORY.append((module, next_pulse, dest))
            counts[next_pulse] += 1

    return counts
            




def _process_inputs(steps, broadcast, low_count=0, high_count=0):
    for prefix, module, destination in steps:
        #print(f"== Operation {prefix} {module} -> {destination}")
        counts = send_pulse(prefix, module, destination)
        low_count += counts[False]
        high_count += counts[True]
    return low_count, high_count


def process_inputs(destinations, start="broadcaster", pulse=False, counts=[0,0]):
    """ We are at start and sending pulse to destinations[start] """
    if start not in destinations:
        return counts

    curr_type = destinations[start]["type"]
    curr_dest = destinations[start]["destinations"]

    if not curr_dest:
        return counts

    next_dests = {}
    conj = {}

    # Send pulse to each destination
    for dest in curr_dest:
        print(f"  {start} -{PULSES_REV[pulse]}-> {dest}")
        MODULES[dest] = pulse
        if pulse:
            counts[1] += 1
        else:
            counts[0] += 1

        if dest not in destinations:
            continue

        dest_type = destinations[dest]["type"]
        if dest_type == "&":
            conj[dest] = dict(destinations[dest], **{"pulse": pulse})
        elif dest_type == "%" and not pulse:
            next_dests[dest] = dict(destinations[dest], **{"pulse": True})
        elif dest_type == "%" and pulse:
            next_dests[dest] = dict(destinations[dest], **{"pulse": not pulse})

    #print("CONJUNCTIONS --")
    # execute conjuctions now
    for con in conj:
        for dest in conj[con]["destinations"]:
            print(f"  {con} -{PULSES_REV[conj[con]['pulse']]}-> {dest}")
            MODULES[dest] = conj[con]['pulse']
            if conj[con]['pulse']:
                counts[1] += 1
            else:
                counts[0] += 1

            if dest in destinations:
                next_dests[dest] = dict(destinations[dest], **{"pulse": pulse})

    #print("NEXT DESTS --")
    # Parsing next inputs in case of 
    for dest in next_dests:
        if dest == start:
            continue
        counts = list(map(add, counts, process_inputs(destinations, dest, next_dests[dest]["pulse"])))

    return counts



def solution_part1(filename):
    """ PART 1
    """
    with open(filename, "r", encoding="utf-8") as file:
        # destinations = {"type": prefix, "destinations": []}
        order = []
        destinations = {}
        for _line in file:
            line = _line.rstrip()
            left, destination = line.split(' -> ')
            prefix = left[0]
            module = left[1:] if prefix != 'b' else left
            destinations[module] = {"type": prefix, "destinations": destination.split(', ')}

        lows, highs = 0, 0
        for i in range(1):
            print(f"button -low-> broadcaster")
            lows += 1
            lows, highs = process_inputs(destinations, "broadcaster", PULSES["low"], [lows, highs])

        print(lows, highs)
        return lows * highs

def solution_part2(filename):
    """ PART 2
    """
    with open(filename, "r", encoding="utf-8") as file:
        for _line in file:
            line = _line.rstrip()


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
    print(solution_part2(f"{INPUT_PATH}/input.{FILENAME_TRUNC}{FILENAME_PART2_EXT}.txt"))
