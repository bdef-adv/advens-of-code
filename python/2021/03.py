#!/usr/bin/python

from pathlib import Path
import sys

INPUT_PATH = str(Path(__file__).parent.resolve()) + '/inputs'
FILENAME = sys.argv[0]
FILENAME_TRUNC = Path(FILENAME).stem
FILENAME_PART2_EXT = ""

def solution_part1(filename):
    with open(filename, "r") as file:
        count_by_position = None

        for _line in file:
            line = _line.rstrip()
            if count_by_position is None:
                count_by_position = [[0,0] for i in range(len(line))]

            for x, ch in enumerate(line):
                if ch == '0':
                    count_by_position[x][0] += 1
                elif ch == '1':
                    count_by_position[x][1] += 1

        gamma = ""
        epsilon = ""
        for zero, one in count_by_position:
            if one > zero:
                gamma += "1"
                epsilon += "0"
            else:
                gamma += "0"
                epsilon += "1"

        return int(gamma, 2) * int(epsilon, 2)


def count_bits_by_position(array):
    count_by_position = None

    for value in array:
        if count_by_position is None:
            count_by_position = [[0,0] for i in range(len(value))]

        for x, ch in enumerate(value):
            if ch == "0":
                count_by_position[x][0] += 1
            elif ch == "1":
                count_by_position[x][1] += 1
                
    return count_by_position

def solution_part2(filename):
    with open(filename, "r") as file:
        lines = []
        count_by_position = None

        # Count bits per position
        for _line in file:
            line = _line.rstrip()
            lines.append(line)
            if count_by_position is None:
                count_by_position = [[0,0] for i in range(len(line))]

            for x, ch in enumerate(line):
                if ch == '0':
                    count_by_position[x][0] += 1
                elif ch == '1':
                    count_by_position[x][1] += 1


        # At first, oxygen_ratings contains all the numbers, we then remove all offending numbers
        oxygen_ratings = lines
        count_by_position_oxy = count_by_position
        # Oxygen rating loop
        # For each bit positions from left to right
        x = 0
        while len(oxygen_ratings) > 1:
            zero, one = count_by_position_oxy[x]
            # find bit_criteria
            bit_criteria_oxy = "1"
            if zero > one:
                bit_criteria_oxy = "0"

            oxygen_to_remove = []
            # Find numbers that are not okay and remove them from the list
            for y, number in enumerate(oxygen_ratings):
                if number[x] != bit_criteria_oxy:
                    oxygen_to_remove.append(y)

            # Recreate new list of numbers without incorrect ones
            new_oxy = []
            for index, line in enumerate(oxygen_ratings):
                if index not in oxygen_to_remove:
                    new_oxy.append(line)

            oxygen_ratings = new_oxy
            count_by_position_oxy = count_bits_by_position(oxygen_ratings)
            #print(oxygen_ratings, count_by_position_oxy)
            x += 1
        
        oxy_rating = oxygen_ratings[0]

        # At first, oxygen_ratings contains all the numbers, we then remove all offending numbers
        co2_ratings = lines
        count_by_position_co2 = count_by_position
        # Oxygen rating loop
        # For each bit positions from left to right
        x = 0
        while len(co2_ratings) > 1:
            zero, one = count_by_position_co2[x]
            # find bit_criteria
            bit_criteria_co2 = "0"
            if zero > one:
                bit_criteria_co2 = "1"

            co2_to_remove = []
            # Find numbers that are not okay and remove them from the list
            for y, number in enumerate(co2_ratings):
                if number[x] != bit_criteria_co2:
                    co2_to_remove.append(y)

            # Recreate new list of numbers without incorrect ones
            new_co2 = []
            for index, line in enumerate(co2_ratings):
                if index not in co2_to_remove:
                    new_co2.append(line)

            co2_ratings = new_co2
            count_by_position_co2 = count_bits_by_position(co2_ratings)
            #print(oxygen_ratings, count_by_position_oxy)
            x += 1
        
        oxy_rating = oxygen_ratings[0]
        co2_rating = co2_ratings[0]

        return int(oxy_rating, 2) * int(co2_rating, 2)



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
